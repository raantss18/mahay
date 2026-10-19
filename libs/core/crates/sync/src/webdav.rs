//! Nextcloud / WebDAV backend: plain `PUT`, `GET`, `PROPFIND`, `DELETE` (and `MKCOL` once) on
//! `<server>/Mahay/<name>`, with HTTP Basic and an app password, over HTTPS (rustls + ring).

use std::time::Duration;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as B64;
use ureq::http::{Request, Response};

use crate::backend::{Backend, BackendError, valid_name};

/// The folder Mahay owns on the server.
pub const FOLDER: &str = "Mahay";

/// A WebDAV server, as the user enters it.
pub struct WebDav {
    /// `<root>/Mahay/`, always ending in `/`.
    folder_url: String,
    auth: String,
    agent: ureq::Agent,
}

impl WebDav {
    /// `root` is the user's WebDAV root, for Nextcloud
    /// `https://<host>/remote.php/dav/files/<user>/`. Plain `http://` is refused except for
    /// `localhost`, so the password and the blobs never travel in clear.
    pub fn new(root: &str, user: &str, app_password: &str) -> Result<Self, BackendError> {
        let root = root.trim();
        let local = ["http://localhost", "http://127.0.0.1", "http://[::1]"]
            .iter()
            .any(|p| root.strip_prefix(p).is_some_and(|r| r.is_empty() || r.starts_with([':', '/'])));
        if !root.starts_with("https://") && !local {
            return Err(BackendError::Other("the WebDAV address must start with https://".into()));
        }
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(300)))
            .timeout_connect(Some(Duration::from_secs(20)))
            .http_status_as_error(false)
            .allow_non_standard_methods(true)
            // never resend the password to wherever a redirect points
            .max_redirects(0)
            .build()
            .into();
        Ok(Self {
            folder_url: format!("{}/{FOLDER}/", root.trim_end_matches('/')),
            auth: format!("Basic {}", B64.encode(format!("{user}:{app_password}"))),
            agent,
        })
    }

    fn url(&self, name: &str) -> Result<String, BackendError> {
        if valid_name(name) { Ok(format!("{}{name}", self.folder_url)) } else { Err(BackendError::Other(format!("bad blob name {name:?}"))) }
    }

    fn send<B: ureq::AsSendBody>(&self, method: &str, url: &str, depth: Option<&str>, body: B) -> Result<Response<ureq::Body>, BackendError> {
        let mut req = Request::builder().method(method).uri(url).header("Authorization", &self.auth);
        if let Some(d) = depth {
            req = req.header("Depth", d).header("Content-Type", "application/xml; charset=utf-8");
        }
        let req = req.body(body).map_err(|e| BackendError::Other(e.to_string()))?;
        self.agent.run(req).map_err(|e| BackendError::Network(e.to_string()))
    }

    fn mkcol(&self) -> Result<(), BackendError> {
        let r = self.send("MKCOL", &self.folder_url, None, ())?;
        // 405: it exists already
        match r.status().as_u16() {
            s if (200..300).contains(&s) || s == 405 => Ok(()),
            s => Err(status_error(s, "MKCOL")),
        }
    }
}

fn status_error(status: u16, what: &str) -> BackendError {
    match status {
        401 | 403 => BackendError::Auth(format!("{what}: HTTP {status}")),
        408 | 425 | 429 | 500..=599 => BackendError::Network(format!("{what}: HTTP {status}")),
        _ => BackendError::Other(format!("{what}: HTTP {status}")),
    }
}

const PROPFIND_BODY: &str = r#"<?xml version="1.0" encoding="utf-8"?><d:propfind xmlns:d="DAV:"><d:prop><d:resourcetype/></d:prop></d:propfind>"#;

impl Backend for WebDav {
    fn put(&self, name: &str, data: &[u8]) -> Result<(), BackendError> {
        let url = self.url(name)?;
        let mut s = self.send("PUT", &url, None, data)?.status().as_u16();
        // 404/409: the Mahay folder does not exist yet
        if s == 404 || s == 409 {
            self.mkcol()?;
            s = self.send("PUT", &url, None, data)?.status().as_u16();
        }
        if (200..300).contains(&s) { Ok(()) } else { Err(status_error(s, "PUT")) }
    }

    fn get(&self, name: &str) -> Result<Option<Vec<u8>>, BackendError> {
        let mut r = self.send("GET", &self.url(name)?, None, ())?;
        match r.status().as_u16() {
            404 | 410 => Ok(None),
            s if (200..300).contains(&s) => {
                r.body_mut().with_config().limit(u64::MAX).read_to_vec().map(Some).map_err(|e| BackendError::Network(e.to_string()))
            }
            s => Err(status_error(s, "GET")),
        }
    }

    fn list(&self) -> Result<Vec<String>, BackendError> {
        let mut r = self.send("PROPFIND", &self.folder_url, Some("1"), PROPFIND_BODY)?;
        match r.status().as_u16() {
            404 => Ok(Vec::new()),
            207 | 200 => {
                let xml = r.body_mut().with_config().limit(64 << 20).read_to_string();
                parse_propfind(&xml.map_err(|e| BackendError::Network(e.to_string()))?)
            }
            s => Err(status_error(s, "PROPFIND")),
        }
    }

    fn delete(&self, name: &str) -> Result<(), BackendError> {
        match self.send("DELETE", &self.url(name)?, None, ())?.status().as_u16() {
            s if (200..300).contains(&s) || s == 404 => Ok(()),
            s => Err(status_error(s, "DELETE")),
        }
    }
}

/// The blob names in a `PROPFIND` (Depth 1) answer: the folder itself and sub-collections are
/// left out, and so is anything that is not a valid blob name.
pub fn parse_propfind(xml: &str) -> Result<Vec<String>, BackendError> {
    let doc = roxmltree::Document::parse(xml).map_err(|e| BackendError::Other(format!("PROPFIND XML: {e}")))?;
    let dav = |n: &roxmltree::Node, name: &str| n.is_element() && n.tag_name().name() == name && n.tag_name().namespace() == Some("DAV:");
    let mut names = Vec::new();
    for resp in doc.descendants().filter(|n| dav(n, "response")) {
        if resp.descendants().any(|n| dav(&n, "collection")) {
            continue;
        }
        let Some(href) = resp.descendants().find(|n| dav(n, "href")).and_then(|n| n.text()) else { continue };
        let last = href.trim().trim_end_matches('/').rsplit('/').next().unwrap_or_default();
        if valid_name(last) {
            names.push(last.to_owned());
        }
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https_only() {
        assert!(WebDav::new("http://cloud.example.org/dav", "u", "p").is_err());
        assert!(WebDav::new("ftp://x", "u", "p").is_err());
        assert!(WebDav::new("http://localhost.evil.org/", "u", "p").is_err());
        let w = WebDav::new("https://cloud.example.org/remote.php/dav/files/alice/", "alice", "app-pass").unwrap();
        assert_eq!(w.folder_url, "https://cloud.example.org/remote.php/dav/files/alice/Mahay/");
        assert_eq!(w.url("manifest.mhy").unwrap(), "https://cloud.example.org/remote.php/dav/files/alice/Mahay/manifest.mhy");
        assert!(w.url("../x").is_err());
        assert_eq!(w.auth, "Basic YWxpY2U6YXBwLXBhc3M=");
        assert!(WebDav::new("http://localhost:8080", "u", "p").is_ok());
    }

    #[test]
    fn nextcloud_propfind() {
        let xml = r#"<?xml version="1.0"?>
<d:multistatus xmlns:d="DAV:" xmlns:s="http://sabredav.org/ns" xmlns:oc="http://owncloud.org/ns" xmlns:nc="http://nextcloud.org/ns">
 <d:response><d:href>/remote.php/dav/files/alice/Mahay/</d:href>
  <d:propstat><d:prop><d:resourcetype><d:collection/></d:resourcetype></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>
 <d:response><d:href>/remote.php/dav/files/alice/Mahay/manifest.mhy</d:href>
  <d:propstat><d:prop><d:resourcetype/></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>
 <d:response><d:href>/remote.php/dav/files/alice/Mahay/0123456789abcdef0123456789abcdef.mhy</d:href>
  <d:propstat><d:prop><d:resourcetype/></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>
 <d:response><d:href>/remote.php/dav/files/alice/Mahay/sub/</d:href>
  <d:propstat><d:prop><d:resourcetype><d:collection/></d:resourcetype></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>
 <d:response><d:href>/remote.php/dav/files/alice/Mahay/my%20notes.txt</d:href>
  <d:propstat><d:prop><d:resourcetype/></d:prop><d:status>HTTP/1.1 200 OK</d:status></d:propstat></d:response>
</d:multistatus>"#;
        assert_eq!(parse_propfind(xml).unwrap(), ["manifest.mhy", "0123456789abcdef0123456789abcdef.mhy"]);
        assert!(parse_propfind("<oops").is_err());
    }
}
