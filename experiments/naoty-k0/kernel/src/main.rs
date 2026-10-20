//! naoty-kernel: a minimal Jupyter kernel in Rust embedding CPython (pyo3), speaking the Jupyter
//! messaging protocol over the pure-Rust `zeromq` crate. K0 spike: feasibility and size, not
//! completeness. No interrupt, no completion, no stdin, no comms.
use std::sync::Arc;

use anyhow::Result;
use jupyter_protocol::{
    ConnectionInfo, ExecuteReply, ExecutionCount, ExecuteInput, JupyterMessage, JupyterMessageContent,
    KernelInfoReply, LanguageInfo, ReplyStatus, ShutdownReply, Status, StreamContent,
};
use jupyter_zmq_client::{
    create_kernel_control_connection, create_kernel_heartbeat_connection,
    create_kernel_iopub_connection, create_kernel_shell_connection,
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

enum Out {
    Stream(bool, String), // is_stderr
    Result(String),
    Display(String),
    Error(String),
}

/// Replaces sys.stdout / sys.stderr: forwards writes to the iopub task as they happen.
#[pyclass]
struct Sink {
    tx: UnboundedSender<Out>,
    err: bool,
}

#[pymethods]
impl Sink {
    fn write(&self, s: &str) -> usize {
        let _ = self.tx.send(Out::Stream(self.err, s.to_string()));
        s.chars().count()
    }
    fn flush(&self) {}
    fn isatty(&self) -> bool {
        false
    }
}

const KERNEL_PY: &str = include_str!("kernel.py");

fn execute(py_mod: &Py<PyModule>, code: String, tx: UnboundedSender<Out>) -> bool {
    Python::attach(|py| {
        let m = py_mod.bind(py);
        let sys = py.import("sys").unwrap();
        let _ = sys.setattr("stdout", Py::new(py, Sink { tx: tx.clone(), err: false }).unwrap());
        let _ = sys.setattr("stderr", Py::new(py, Sink { tx: tx.clone(), err: true }).unwrap());
        let tx2 = tx.clone();
        let emit = pyo3::types::PyCFunction::new_closure(py, None, None, move |a, _| {
            let kind: String = a.get_item(0)?.extract()?;
            let payload: String = a.get_item(1)?.extract()?;
            let _ = tx2.send(match kind.as_str() {
                "result" => Out::Result(payload),
                "display" => Out::Display(payload),
                _ => Out::Error(payload),
            });
            PyResult::Ok(())
        })
        .unwrap();
        let ok = m.getattr("run").unwrap().call1((code, emit)).and_then(|r| r.extract::<bool>()).unwrap_or(false);
        let _ = sys.setattr("stdout", sys.getattr("__stdout__").unwrap());
        let _ = sys.setattr("stderr", sys.getattr("__stderr__").unwrap());
        ok
    })
}

fn msg(parent: &JupyterMessage, c: impl Into<JupyterMessage>) -> JupyterMessage {
    let mut m: JupyterMessage = c.into();
    m = m.with_parent(parent);
    m
}

#[tokio::main]
async fn main() -> Result<()> {
    let t0 = std::time::Instant::now();
    let path = std::env::args().nth(2).or_else(|| std::env::args().nth(1)).expect("connection file");
    let info: ConnectionInfo = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    let session = uuid_like();

    // Python: start and load the helper module once.
    let py_mod: Arc<Py<PyModule>> = Arc::new(Python::attach(|py| {
        let _ = py.import("sys").and_then(|s| s.getattr("path")).and_then(|p| {
            if let Ok(extra) = std::env::var("NAOTY_PYPATH") {
                for e in extra.split(':') {
                    p.call_method1("insert", (0, e))?;
                }
            }
            Ok(())
        });
        PyModule::from_code(py, &std::ffi::CString::new(KERNEL_PY).unwrap(), c"naoty_kernel.py", c"naoty_kernel")
            .unwrap()
            .unbind()
    }));
    let trace = std::env::var("NAOTY_TRACE").is_ok();
    if trace { eprintln!("[kernel] python initialised, helper loaded: {} ms", t0.elapsed().as_millis()); }
    let py_version = Python::attach(|py| py.version().split_whitespace().next().unwrap_or("").to_string());

    let mut hb = create_kernel_heartbeat_connection(&info).await?;
    let mut iopub = create_kernel_iopub_connection(&info, &session).await?;
    let mut shell = create_kernel_shell_connection(&info, &session).await?;
    let mut control = create_kernel_control_connection(&info, &session).await?;

    if trace { eprintln!("[kernel] sockets bound, ready: {} ms", t0.elapsed().as_millis()); }
    tokio::spawn(async move { while hb.single_heartbeat().await.is_ok() {} });

    let kinfo = |parent: &JupyterMessage| -> JupyterMessage {
        msg(
            parent,
            KernelInfoReply {
                status: ReplyStatus::Ok,
                protocol_version: "5.3".into(),
                implementation: "naoty-kernel".into(),
                implementation_version: env!("CARGO_PKG_VERSION").into(),
                language_info: LanguageInfo {
                    name: "python".into(),
                    version: py_version.clone(),
                    mimetype: Some("text/x-python".into()),
                    file_extension: Some(".py".into()),
                    pygments_lexer: Some("ipython3".into()),
                    codemirror_mode: None,
                    nbconvert_exporter: Some("python".into()),
                },
                banner: format!("Naoty K0 kernel, CPython {py_version}"),
                help_links: vec![],
                debugger: false,
                error: None,
            },
        )
    };

    // control: shutdown (and kernel_info) on their own task.
    {
        let kinfo_msgs = std::sync::Mutex::new(());
        let _ = kinfo_msgs;
        tokio::spawn(async move {
            while let Ok(m) = control.read().await {
                if let JupyterMessageContent::ShutdownRequest(r) = &m.content {
                    let _ = control
                        .send(msg(&m, ShutdownReply { restart: r.restart, status: ReplyStatus::Ok, error: None }))
                        .await;
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    std::process::exit(0);
                }
            }
        });
    }

    iopub.send(Status::starting().into()).await?;
    let mut count = 0usize;
    while let Ok(m) = shell.read().await {
        match &m.content {
            JupyterMessageContent::KernelInfoRequest(_) => shell.send(kinfo(&m)).await?,
            JupyterMessageContent::ExecuteRequest(req) => {
                count += 1;
                iopub.send(msg(&m, Status::busy())).await?;
                iopub
                    .send(msg(&m, ExecuteInput { code: req.code.clone(), execution_count: ExecutionCount::new(count) }))
                    .await?;
                let (tx, mut rx) = unbounded_channel();
                let pm = py_mod.clone();
                let code = req.code.clone();
                let job = tokio::task::spawn_blocking(move || execute(&pm, code, tx));
                let mut job = job;
                let mut ok = false;
                let mut done = false;
                loop {
                    // Sinks may outlive the cell (logging keeps sys.stderr), so the channel never
                    // closes: once the job is done, drain what is queued and stop.
                    let o = if done {
                        match rx.try_recv() { Ok(o) => o, Err(_) => break }
                    } else {
                        tokio::select! {
                            o = rx.recv() => match o { Some(o) => o, None => break },
                            r = &mut job => { ok = r.unwrap_or(false); done = true; continue; }
                        }
                    };
                    let c: JupyterMessageContent = match o {
                        Out::Stream(e, t) => StreamContent { name: if e { jupyter_protocol::Stdio::Stderr } else { jupyter_protocol::Stdio::Stdout }, text: t }.into(),
                        Out::Result(j) => {
                            let mut v: serde_json::Value = serde_json::from_str(&j)?;
                            v["execution_count"] = count.into();
                            JupyterMessageContent::ExecuteResult(serde_json::from_value(v)?)
                        }
                        Out::Display(j) => JupyterMessageContent::DisplayData(serde_json::from_value(serde_json::from_str(&j)?)?),
                        Out::Error(j) => JupyterMessageContent::ErrorOutput(serde_json::from_str(&j)?),
                    };
                    iopub.send(msg(&m, c)).await?;
                }
                if !done { ok = job.await.unwrap_or(false); }
                let reply = if ok {
                    ExecuteReply { status: ReplyStatus::Ok, execution_count: ExecutionCount::new(count), ..Default::default() }
                } else {
                    ExecuteReply { status: ReplyStatus::Error, execution_count: ExecutionCount::new(count), ..Default::default() }
                };
                shell.send(msg(&m, reply)).await?;
                iopub.send(msg(&m, Status::idle())).await?;
            }
            JupyterMessageContent::ShutdownRequest(r) => {
                shell.send(msg(&m, ShutdownReply { restart: r.restart, status: ReplyStatus::Ok, error: None })).await?;
                break;
            }
            _ => {}
        }
    }
    let _ = PyDict::new; // keep import used on all configs
    Ok(())
}

fn uuid_like() -> String {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    format!("naoty-{t:x}-{}", std::process::id())
}
