//! Joining over `wss://`: a guest and a managing connection reach a host through a TLS terminator
//! (like the reverse proxy in front of a server) whose certificate comes from a test authority
//! only test builds trust; a certificate for another name is refused with a clear reason.

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use figcraft_engine::live::{Event, Status};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::{ServerConfig, ServerConnection, StreamOwned};

use crate::guest::{Guest, GuestConfig};
use crate::invite::{Endpoint, Invite};
use crate::loopback_tests::{Probe, fast_limits, quick, start};
use crate::manage;

/// Serve `localhost`'s test certificate on a port of `localhost` and relay each connection to
/// `upstream`.
fn terminator(upstream: SocketAddr) -> SocketAddr {
    let cert = CertificateDer::from(&include_bytes!("testdata/localhost.der")[..]).into_owned();
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(include_bytes!("testdata/localhost-key.der").to_vec()));
    let cfg = ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)
        .unwrap();
    let cfg = Arc::new(cfg);
    // The name's first address, which a client tries first (Windows takes seconds to give up on
    // a refused one).
    let listener = TcpListener::bind("localhost:0").unwrap();
    let addr = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for s in listener.incoming().flatten() {
            let cfg = cfg.clone();
            std::thread::spawn(move || relay(s, upstream, cfg));
        }
    });
    addr
}

/// Copy bytes both ways until either side ends (polling with short timeouts: test code).
fn relay(s: TcpStream, upstream: SocketAddr, cfg: Arc<ServerConfig>) {
    let tick = Some(Duration::from_millis(2));
    let Ok(mut up) = TcpStream::connect(upstream) else { return };
    up.set_read_timeout(tick).unwrap();
    s.set_read_timeout(tick).unwrap();
    let mut tls = StreamOwned::new(ServerConnection::new(cfg).unwrap(), s);
    let mut buf = [0u8; 16 * 1024];
    let pass = |r: io::Result<usize>, to: &mut dyn Write, buf: &[u8]| match r {
        Ok(0) => false,
        Ok(n) => to.write_all(&buf[..n]).and_then(|()| to.flush()).is_ok(),
        Err(e) => matches!(e.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut),
    };
    loop {
        let r = tls.read(&mut buf);
        if !pass(r, &mut up, &buf) {
            return;
        }
        let r = up.read(&mut buf);
        if !pass(r, &mut tls, &buf) {
            return;
        }
    }
}

fn secure(host: String, invite: &Invite) -> Invite {
    Invite { server: Endpoint { host, path: "/".into(), secure: true }, ..invite.clone() }
}

#[test]
fn guests_and_managing_connections_join_over_wss() {
    let (mut host, _hp, invite) = start();
    let front = terminator(host.addr());
    let wss = secure(format!("localhost:{}", front.port()), &invite);
    assert!(wss.url().starts_with("wss://localhost:"));
    let mut probe = Probe::new();
    let mut g = Guest::start(quick(GuestConfig::new(wss.clone(), "Ada".into())), probe.inbox.clone()).unwrap();
    probe.wait("the welcome over TLS", |e| e.iter().any(|e| matches!(e, Event::Welcome { .. })));
    // The managing path speaks TLS too: an app's host answers, and refuses to manage files.
    let err = manage::request(&wss.server, "key", Vec::new(), &fast_limits(), Duration::from_secs(5)).unwrap_err();
    assert!(err.contains("hosted by an app"), "{err}");
    g.leave();
    host.stop();
}

#[test]
fn a_certificate_for_another_name_is_refused() {
    let (mut host, _hp, invite) = start();
    let front = terminator(host.addr());
    let mut probe = Probe::new();
    let _g = Guest::start(quick(GuestConfig::new(secure(front.to_string(), &invite), "Ada".into())), probe.inbox.clone()).unwrap();
    probe.wait("the refusal", |e| {
        e.iter()
            .any(|e| matches!(e, Event::Status(Status::Failed { reason }) if reason.contains(&format!("certificate of {front} is for another name"))))
    });
    host.stop();
}

#[test]
fn reader_and_writer_share_one_session_without_blocking_each_other() {
    // An echo server behind the terminator: more than any socket buffers hold, written in one go,
    // comes back while it is still being sent (were the reader to wait for the writer, both would
    // stall once the buffers fill).
    let echo = TcpListener::bind("127.0.0.1:0").unwrap();
    let upstream = echo.local_addr().unwrap();
    std::thread::spawn(move || {
        let (mut s, _) = echo.accept().unwrap();
        let _ = io::copy(&mut s.try_clone().unwrap(), &mut s);
    });
    let front = terminator(upstream);
    let server = Endpoint { host: format!("localhost:{}", front.port()), path: "/".into(), secure: true };
    let (mut rd, mut wr, _sock) = crate::net::open(&server, Duration::from_secs(5), &fast_limits()).unwrap();
    rd.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    let sent: Vec<u8> = (0..8 << 20).map(|i: u32| (i % 251) as u8).collect();
    let expect = sent.clone();
    let writer = std::thread::spawn(move || wr.write_all(&sent).unwrap());
    let mut got = vec![0u8; expect.len()];
    rd.read_exact(&mut got).unwrap();
    writer.join().unwrap();
    assert!(got == expect);
}
