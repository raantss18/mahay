//! K0 client: spawn (or connect to) a Jupyter kernel through jupyter-zmq-client, run cells,
//! print every message received, and time startup / first result.
//!
//! usage:
//!   naoty-k0-client spawn <argv...>     argv contains `{conn}` replaced by the connection file
//!   naoty-k0-client connect <file.json> connect to an already running kernel
//!   naoty-k0-client nbfmt <file.ipynb>  nbformat round trip check
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use jupyter_protocol::{
    connection_info::Transport, ConnectionInfo, ExecuteRequest, JupyterMessage,
    JupyterMessageContent, KernelInfoRequest, ShutdownRequest,
};
use jupyter_zmq_client::{
    create_client_control_connection, create_client_iopub_connection,
    create_client_shell_connection_with_identity, peek_ports, peer_identity_for_session,
};
use tokio::time::timeout;

const CELLS: &[&str] = &[
    "print(1+1)",
    "1+1",
    "import matplotlib.pyplot as plt\nplt.plot([1,2,3],[1,4,9])\nplt.show()",
    "1/0",
    "import numpy as np, sympy as sp\nx=sp.symbols('x')\nprint(np.arange(6).reshape(2,3).sum(), sp.integrate(x**2, x))",
];

fn summarize(m: &JupyterMessage) -> String {
    let v = serde_json::to_value(&m.content).unwrap_or_default();
    let mut s = v.to_string();
    if s.len() > 160 {
        s.truncate(160);
        s.push('…');
    }
    format!("{:<14} {}", m.content.message_type(), s)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("spawn") => {
            let t0 = Instant::now();
            let ip = "127.0.0.1".parse().unwrap();
            let ports = peek_ports(ip, 5).await?;
            let info = ConnectionInfo {
                transport: Transport::TCP,
                ip: "127.0.0.1".into(),
                stdin_port: ports[0],
                control_port: ports[1],
                hb_port: ports[2],
                shell_port: ports[3],
                iopub_port: ports[4],
                signature_scheme: "hmac-sha256".into(),
                key: uuid::Uuid::new_v4().to_string(),
                kernel_name: Some("python3".into()),
            };
            let dir = std::env::temp_dir().join(format!("naoty-k0-{}", std::process::id()));
            std::fs::create_dir_all(&dir)?;
            let conn = dir.join("kernel.json");
            std::fs::write(&conn, serde_json::to_string(&info)?)?;
            let argv: Vec<String> = args[1..]
                .iter()
                .map(|a| a.replace("{conn}", conn.to_str().unwrap()))
                .collect();
            let mut child = tokio::process::Command::new(&argv[0])
                .args(&argv[1..])
                .kill_on_drop(true)
                .spawn()?;
            // Connecting a ZeroMQ socket before the kernel has bound costs the zeromq crate's
            // reconnect backoff (~1.1 s). Wait until the shell port accepts TCP, then connect.
            while tokio::net::TcpStream::connect(("127.0.0.1", info.shell_port)).await.is_err() {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
            println!("TIMING shell port bound after {} ms", t0.elapsed().as_millis());
            let r = run(&info, t0).await;
            let _ = child.start_kill();
            let _ = child.wait().await;
            r
        }
        Some("connect") => {
            let info: ConnectionInfo = serde_json::from_str(&std::fs::read_to_string(&args[1])?)?;
            run(&info, Instant::now()).await
        }
        Some("nbfmt") => nbfmt(&args[1]),
        Some("ws") => ws(&args[1], &args[2]).await,
        _ => bail!("see source for usage"),
    }
}

async fn run(info: &ConnectionInfo, t0: Instant) -> Result<()> {
    let session = uuid::Uuid::new_v4().to_string();
    let id = peer_identity_for_session(&session)?;
    let mut iopub = create_client_iopub_connection(info, "", &session).await?;
    let mut shell = create_client_shell_connection_with_identity(info, &session, id).await?;
    let mut control = create_client_control_connection(info, &session).await?;

    // Wait until the kernel answers kernel_info (retry: the kernel may not be bound yet).
    let mut ready = None;
    for _ in 0..200 {
        shell.send(KernelInfoRequest {}.into()).await?;
        if let Ok(Ok(m)) = timeout(Duration::from_millis(250), shell.read()).await {
            if let JupyterMessageContent::KernelInfoReply(r) = &m.content {
                println!(
                    "kernel_info_reply: {} {} / {} {}",
                    r.implementation, r.implementation_version, r.language_info.name, r.protocol_version
                );
                ready = Some(t0.elapsed());
                break;
            }
        }
    }
    let Some(ready) = ready else { bail!("kernel never answered") };
    println!("TIMING kernel ready (spawn → kernel_info_reply): {} ms", ready.as_millis());

    for (n, code) in CELLS.iter().enumerate() {
        println!("--- cell {n}: {:?}", code);
        let t = Instant::now();
        let req: JupyterMessage = ExecuteRequest::new((*code).into()).into();
        let req_id = req.header.msg_id.clone();
        shell.send(req).await?;
        let mut first = None;
        let mut got_reply = false;
        let mut idle = false;
        while !(got_reply && idle) {
            tokio::select! {
                m = iopub.read() => {
                    let m = m?;
                    if m.parent_header.as_ref().map(|h| &h.msg_id) != Some(&req_id) { continue; }
                    if matches!(m.content, JupyterMessageContent::ExecuteResult(_)
                        | JupyterMessageContent::StreamContent(_) | JupyterMessageContent::DisplayData(_)
                        | JupyterMessageContent::ErrorOutput(_)) && first.is_none() {
                        first = Some(t.elapsed());
                    }
                    println!("iopub  {}", summarize(&m));
                    if let JupyterMessageContent::Status(s) = &m.content {
                        if format!("{:?}", s.execution_state) == "Idle" { idle = true; }
                    }
                }
                m = shell.read() => {
                    let m = m?;
                    println!("shell  {}", summarize(&m));
                    got_reply = true;
                }
                _ = tokio::time::sleep(Duration::from_secs(60)) => bail!("timeout on cell {n}"),
            }
        }
        println!("TIMING cell {n}: first output {:?} ms, total {} ms", first.map(|d| d.as_millis()), t.elapsed().as_millis());
    }

    control.send(ShutdownRequest { restart: false }.into()).await?;
    if let Ok(Ok(m)) = timeout(Duration::from_secs(5), control.read()).await {
        println!("control {}", summarize(&m));
    }
    Ok(())
}

fn nbfmt(path: &str) -> Result<()> {
    let text = std::fs::read_to_string(path)?;
    let nb = nbformat::parse_notebook(&text)?;
    let out = nbformat::serialize_notebook(&nb)?;
    let a: serde_json::Value = serde_json::from_str(&text)?;
    let b: serde_json::Value = serde_json::from_str(&out)?;
    if a == b {
        println!("ROUNDTRIP identical (JSON-equal), {} bytes in, {} bytes out", text.len(), out.len());
    } else {
        println!("ROUNDTRIP DIFFERS");
        diff("", &a, &b);
    }
    Ok(())
}

fn diff(p: &str, a: &serde_json::Value, b: &serde_json::Value) {
    use serde_json::Value::*;
    match (a, b) {
        (Object(x), Object(y)) => {
            for k in x.keys().chain(y.keys()).collect::<std::collections::BTreeSet<_>>() {
                match (x.get(k), y.get(k)) {
                    (Some(u), Some(v)) => diff(&format!("{p}/{k}"), u, v),
                    (Some(u), None) => println!("  lost   {p}/{k} = {}", trunc(u)),
                    (None, Some(v)) => println!("  added  {p}/{k} = {}", trunc(v)),
                    _ => {}
                }
            }
        }
        (Array(x), Array(y)) if x.len() == y.len() => {
            for (i, (u, v)) in x.iter().zip(y).enumerate() {
                diff(&format!("{p}/{i}"), u, v)
            }
        }
        _ if a != b => println!("  changed {p}: {} -> {}", trunc(a), trunc(b)),
        _ => {}
    }
}

fn trunc(v: &serde_json::Value) -> String {
    let mut s = v.to_string();
    s.truncate(80);
    s
}

/// Remote kernel: jupyter-websocket-client against a `jupyter server` (token auth).
async fn ws(base: &str, token: &str) -> Result<()> {
    use futures::{SinkExt, StreamExt};
    let t0 = Instant::now();
    let http = reqwest::Client::new();
    let server = jupyter_websocket_client::RemoteServer { base_url: base.into(), token: token.into() };
    // REST goes through reqwest with the token in a header (the crate itself puts it in the URL).
    let kernel: serde_json::Value = http
        .post(format!("{base}/api/kernels"))
        .header("Authorization", format!("token {token}"))
        .json(&serde_json::json!({"name": "python3"}))
        .send().await?.error_for_status()?.json().await?;
    let kid = kernel["id"].as_str().unwrap().to_string();
    println!("kernel launched through REST: {kid} ({} ms)", t0.elapsed().as_millis());
    let (sock, _resp) = server.connect_to_kernel(&kid).await?;
    let (mut w, mut r) = sock.split();
    w.send(KernelInfoRequest {}.into()).await?;
    while let Some(m) = r.next().await.transpose()? {
        if let JupyterMessageContent::KernelInfoReply(k) = &m.content {
            println!("kernel_info_reply: {} {}", k.implementation, k.implementation_version);
            break;
        }
    }
    println!("TIMING remote kernel ready (POST /api/kernels → kernel_info_reply): {} ms", t0.elapsed().as_millis());
    for (n, code) in CELLS.iter().enumerate() {
        println!("--- cell {n}: {:?}", code);
        let t = Instant::now();
        let req: JupyterMessage = ExecuteRequest::new((*code).into()).into();
        let id = req.header.msg_id.clone();
        w.send(req).await?;
        let (mut reply, mut idle, mut first) = (false, false, None);
        while !(reply && idle) {
            let m = tokio::time::timeout(Duration::from_secs(60), r.next()).await?.transpose()?.ok_or_else(|| anyhow::anyhow!("closed"))?;
            if m.parent_header.as_ref().map(|h| &h.msg_id) != Some(&id) { continue; }
            match &m.content {
                JupyterMessageContent::ExecuteReply(_) => reply = true,
                JupyterMessageContent::Status(s) if format!("{:?}", s.execution_state) == "Idle" => idle = true,
                JupyterMessageContent::ExecuteResult(_) | JupyterMessageContent::StreamContent(_)
                | JupyterMessageContent::DisplayData(_) | JupyterMessageContent::ErrorOutput(_) => { first.get_or_insert(t.elapsed()); }
                _ => {}
            }
            println!("ws     {}", summarize(&m));
        }
        println!("TIMING cell {n}: first output {:?} ms, total {} ms", first.map(|d| d.as_millis()), t.elapsed().as_millis());
    }
    http.delete(format!("{base}/api/kernels/{kid}")).header("Authorization", format!("token {token}")).send().await?;
    Ok(())
}
