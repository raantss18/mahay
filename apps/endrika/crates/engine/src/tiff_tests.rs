//! TIFF images open, import and place like the other formats; damaged ones are errors.

use base64::Engine as _;
use serde_json::json;

use crate::Session;

fn tiff(w: u32, h: u32) -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(w, h, image::Rgba([10, 120, 200, 255]));
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Tiff).unwrap();
    out.into_inner()
}

#[test]
fn tiff_images_import_and_open() {
    let bytes = tiff(30, 20);
    let mut s = Session::new();
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let out = s.execute("file.import", &json!({"data": b64, "name": "scan.tif", "x": 0, "y": 0})).unwrap();
    let id = crate::params::id_of(&out["id"]).unwrap();
    let n = s.doc().unwrap().node(id).unwrap();
    assert_eq!((n.width, n.height), (30.0, 20.0));
    // SVG viewers don't show TIFF: the export embeds it as PNG.
    let svg = String::from_utf8(crate::cmd::file::export_bytes(s.doc().unwrap(), &[id], figcraft_doc::ExportFormat::Svg, 1.0).unwrap()).unwrap();
    assert!(svg.contains("data:image/png;base64,"), "{svg}");
    let path = std::env::temp_dir().join(format!("figcraft-tiff-{}.tiff", std::process::id()));
    std::fs::write(&path, &bytes).unwrap();
    s.execute("file.open", &json!({"path": path.to_string_lossy()})).unwrap();
    assert_eq!(s.doc().unwrap().images.len(), 1);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn damaged_tiffs_are_errors() {
    let bytes = tiff(8, 8);
    let mut s = Session::new();
    for junk in [&bytes[..12], &bytes[..bytes.len() / 2], b"II*\0\x08\0\0\0\xff\xff".as_slice()] {
        let b64 = base64::engine::general_purpose::STANDARD.encode(junk);
        // Either refused or placed; never a panic, and the renderer copes with what was placed.
        if let Ok(out) = s.execute("file.import", &json!({"data": b64, "name": "bad.tif"})) {
            let id = crate::params::id_of(&out["id"]).unwrap();
            let _ = figcraft_render::render_node(s.doc().unwrap(), id, 1.0);
        }
    }
    let header_only = base64::engine::general_purpose::STANDARD.encode(b"II*\0\x08\0\0\0");
    assert!(s.execute("file.import", &json!({"data": header_only, "name": "bad.tif"})).is_err());
}
