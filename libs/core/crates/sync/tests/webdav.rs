//! The WebDAV backend against a tiny in-process WebDAV server on 127.0.0.1, then a full sync
//! through it.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use mahay_sync::{Backend, BackendError, Config, Engine, WebDav, open_key_with};

const AUTH: &str = "Basic YWxpY2U6c2VjcmV0"; // alice:secret

#[derive(Default)]
struct Server {
    files: BTreeMap<String, Vec<u8>>,
    folder: bool,
    methods: Vec<String>,
}

/// Serves `/dav/Mahay/<name>` until the test ends. Returns the WebDAV root URL.
fn serve(state: Arc<Mutex<Server>>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://127.0.0.1:{}/dav/", listener.local_addr().unwrap().port());
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { return };
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap_or(0) == 0 {
                continue;
            }
            let mut parts = line.split_whitespace();
            let (method, path) = (parts.next().unwrap().to_owned(), parts.next().unwrap().to_owned());
            let (mut len, mut auth) = (0usize, String::new());
            loop {
                let mut h = String::new();
                reader.read_line(&mut h).unwrap();
                let h = h.trim_end();
                if h.is_empty() {
                    break;
                }
                let (k, v) = h.split_once(':').unwrap();
                match k.to_ascii_lowercase().as_str() {
                    "content-length" => len = v.trim().parse().unwrap(),
                    "authorization" => auth = v.trim().to_owned(),
                    _ => {}
                }
            }
            let mut body = vec![0; len];
            reader.read_exact(&mut body).unwrap();

            let mut s = state.lock().unwrap();
            s.methods.push(method.clone());
            let name = path.strip_prefix("/dav/Mahay/").map(str::to_owned);
            let (status, out): (u16, Vec<u8>) = if auth != AUTH {
                (401, Vec::new())
            } else {
                match (method.as_str(), name.as_deref()) {
                    ("MKCOL", Some("")) => {
                        let was = s.folder;
                        s.folder = true;
                        (if was { 405 } else { 201 }, Vec::new())
                    }
                    ("PUT", Some(n)) if !s.folder => {
                        let _ = n;
                        (409, Vec::new())
                    }
                    ("PUT", Some(n)) => {
                        s.files.insert(n.to_owned(), body);
                        (201, Vec::new())
                    }
                    ("GET", Some(n)) => match s.files.get(n) {
                        Some(d) => (200, d.clone()),
                        None => (404, Vec::new()),
                    },
                    ("DELETE", Some(n)) => (if s.files.remove(n).is_some() { 204 } else { 404 }, Vec::new()),
                    ("PROPFIND", Some("")) if !s.folder => (404, Vec::new()),
                    ("PROPFIND", Some("")) => {
                        let mut xml = String::from(
                            r#"<?xml version="1.0"?><d:multistatus xmlns:d="DAV:"><d:response><d:href>/dav/Mahay/</d:href><d:propstat><d:prop><d:resourcetype><d:collection/></d:resourcetype></d:prop></d:propstat></d:response>"#,
                        );
                        for n in s.files.keys() {
                            xml += &format!(
                                "<d:response><d:href>/dav/Mahay/{n}</d:href><d:propstat><d:prop><d:resourcetype/></d:prop></d:propstat></d:response>"
                            );
                        }
                        xml += "</d:multistatus>";
                        (207, xml.into_bytes())
                    }
                    _ => (400, Vec::new()),
                }
            };
            drop(s);
            let head = format!("HTTP/1.1 {status} X\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", out.len());
            let _ = stream.write_all(head.as_bytes());
            let _ = stream.write_all(&out);
        }
    });
    url
}

#[test]
fn webdav_put_get_list_delete() {
    let state = Arc::new(Mutex::new(Server::default()));
    let url = serve(state.clone());
    let dav = WebDav::new(&url, "alice", "secret").unwrap();
    assert_eq!(dav.list().unwrap(), Vec::<String>::new());
    assert_eq!(dav.get("manifest.mhy").unwrap(), None);
    let big = vec![9u8; 3 << 20];
    dav.put("a.mhy", &big).unwrap(); // creates Mahay/ on the way
    dav.put("b.mhy", b"b").unwrap();
    assert_eq!(dav.get("a.mhy").unwrap().unwrap(), big);
    assert_eq!(dav.list().unwrap(), ["a.mhy", "b.mhy"]);
    dav.delete("a.mhy").unwrap();
    dav.delete("a.mhy").unwrap();
    assert_eq!(dav.list().unwrap(), ["b.mhy"]);
    assert_eq!(state.lock().unwrap().methods.iter().filter(|m| *m == "MKCOL").count(), 1);

    let wrong = WebDav::new(&url, "alice", "nope").unwrap();
    assert!(matches!(wrong.get("b.mhy"), Err(BackendError::Auth(_))));
}

#[test]
fn unreachable_server_is_a_network_error() {
    let port = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let dav = WebDav::new(&format!("http://127.0.0.1:{port}/"), "u", "p").unwrap();
    assert!(matches!(dav.list(), Err(BackendError::Network(_))));
}

#[test]
fn full_sync_over_webdav() {
    let state = Arc::new(Mutex::new(Server::default()));
    let url = serve(state.clone());
    let device = |name: &str| {
        let dav: Arc<dyn Backend> = Arc::new(WebDav::new(&url, "alice", "secret").unwrap());
        let (key, _) = open_key_with(dav.as_ref(), "phrase", 1000).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let cfg = Config { root: dir.path().join("docs"), state_dir: dir.path().join("state"), device: name.into() };
        std::fs::create_dir_all(&cfg.root).unwrap();
        (Engine::new(cfg, dav, key).unwrap(), dir)
    };
    let (mut a, da) = device("Laptop");
    let (mut b, db) = device("Tablette");
    std::fs::write(da.path().join("docs/hello.txt"), b"salama").unwrap();
    assert_eq!(a.sync().unwrap().uploaded, ["hello.txt"]);
    assert_eq!(b.sync().unwrap().downloaded, ["hello.txt"]);
    assert_eq!(std::fs::read(db.path().join("docs/hello.txt")).unwrap(), b"salama");
    let names: Vec<String> = state.lock().unwrap().files.keys().cloned().collect();
    assert_eq!(names.len(), 3, "{names:?}"); // check.json, manifest.mhy, one blob
}
