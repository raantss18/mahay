//! Command-line parsing: files, the control port and invite links.

use crate::args::{Args, file_url_path, is_invite, parse};

fn args(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

#[test]
fn files_and_the_control_port() {
    assert_eq!(parse(&args(&[])), Args::default());
    let a = parse(&args(&["--control", "4242", "a.endrika", "--verbose", "b.svg"]));
    assert_eq!(a, Args { control: Some(4242), files: args(&["a.endrika", "b.svg"]), ..Args::default() });
    // A missing or bad port is ignored, not taken as a file.
    assert_eq!(parse(&args(&["--control"])), Args::default());
    assert_eq!(parse(&args(&["--control", "nope"])).control, None);
}

#[test]
fn automation_roots_repeat_and_are_never_files() {
    let a = parse(&args(&["--automation-read-root", "in", "--automation-write-root=out", "--automation-read-root=more", "a.endrika"]));
    assert_eq!(a.read_roots, args(&["in", "more"]));
    assert_eq!(a.write_roots, args(&["out"]));
    assert_eq!(a.files, args(&["a.endrika"]));
    // A flag without its folder is kept as an empty one, which startup refuses.
    assert_eq!(parse(&args(&["--automation-write-root"])).write_roots, args(&[""]));
}

#[test]
fn invite_links_join_instead_of_opening() {
    let link = "endrika://join?host=192.168.1.20:47900&file=abc&token=AbCdEfGhIjKlMnOpQrStUv";
    let a = parse(&args(&[link]));
    assert_eq!(a.join.as_deref(), Some(link));
    assert!(a.files.is_empty(), "a link is never opened as a file");
    // How browsers and shells pass it: any case, a slash after `join`, alongside files.
    let a = parse(&args(&["design.endrika", "ENDRIKA://join/?host=h:1&file=f&token=t"]));
    assert_eq!(a.files, args(&["design.endrika"]));
    assert_eq!(a.join.as_deref(), Some("ENDRIKA://join/?host=h:1&file=f&token=t"));
    assert!(is_invite("endrika:join") && !is_invite("endrika.svg") && !is_invite("fig") && !is_invite("ü€endrika:"));
}

#[test]
fn file_urls_are_paths() {
    // What a Linux desktop passes for `%U`.
    let a = parse(&args(&["file:///home/ada/My%20Designs/a+b.endrika", "FILE://localhost/tmp/x.svg"]));
    assert_eq!(a.files, args(&["/home/ada/My Designs/a+b.endrika", "/tmp/x.svg"]), "spaces decoded, plus signs kept");
    assert_eq!(file_url_path("file:///%E2%82%AC.png").as_deref(), Some("/€.png"));
    let win = file_url_path("file:///C:/Designs/a.endrika");
    assert_eq!(win.as_deref(), Some(if cfg!(windows) { "C:/Designs/a.endrika" } else { "/C:/Designs/a.endrika" }));
    for bad in ["file://server/share/a.svg", "file:///broken%2", "file:///bad%zz", "file:///%FF", "file:", "fi", "ü€file:///x"] {
        assert_eq!(file_url_path(bad), None, "{bad}");
    }
}
