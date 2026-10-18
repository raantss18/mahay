//! File I/O shared by the MCP server and the CLI: open/save any supported
//! format, with `.sary` (ZIP or directory bundle) handled natively.

use std::path::Path;

use sary_doc::Document;
use sary_format::{SaryWriter, SaveOptions};
use sary_io::ExportOptions;

use crate::AutomationError;

/// Result of opening a file.
pub struct Opened {
    pub document: Document,
    pub warnings: Vec<String>,
    pub source_read_only: bool,
}

/// Decode a document that was read through a filesystem capability.
pub fn open_bytes(name: &str, bytes: &[u8]) -> Result<Opened, AutomationError> {
    if Path::new(name).extension().is_some_and(|extension| extension.eq_ignore_ascii_case(sary_format::EXTENSION)) {
        return Ok(Opened { document: sary_format::load_from_bytes(bytes)?, warnings: Vec::new(), source_read_only: false });
    }
    let r = sary_io::import(name, bytes)?;
    Ok(Opened { document: r.document, warnings: r.warnings, source_read_only: r.source_read_only })
}

/// Open a document from disk. Directory bundles and `.sary` ZIPs load
/// natively; everything else goes through `sary-io` (PSD, PNG, …).
pub fn open(path: &Path) -> Result<Opened, AutomationError> {
    if path.is_dir() {
        return Ok(Opened { document: sary_format::load_path(path)?, warnings: Vec::new(), source_read_only: false });
    }
    let bytes = sary_format::read_file(path).map_err(|e| AutomationError::Io(format!("{}: {e}", path.display())))?;
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    open_bytes(&name, &bytes)
}

/// Previews for a `.sary` bundle.
pub fn previews(doc: &Document) -> SaveOptions {
    SaveOptions { thumbnail: Some(sary_compose::thumbnail(doc, 256)), composite: Some(sary_compose::thumbnail(doc, 1024)) }
}

/// Encode the name the saved native bundle will report on reopening. Save As updates
/// the session identity only *after* a successful write; without a separate snapshot
/// the manifest serializes the previous name instead.
fn named_for_output(doc: &Document, path: &str) -> Document {
    let mut named = doc.clone();
    if let Some(name) = path.rsplit(['/', '\\']).next().filter(|name| !name.is_empty()) {
        named.name = name.to_owned();
    }
    named
}

/// Save or export `doc` to `path`, choosing the format from the extension
/// (or `format_override`, an extension such as `"png"`). `writer` makes
/// repeated `.sary` saves incremental. Returns warnings about lost data.
pub fn save(
    doc: &Document,
    path: &Path,
    format_override: Option<&str>,
    opts: &ExportOptions,
    writer: Option<&mut SaryWriter>,
) -> Result<Vec<String>, AutomationError> {
    let ext = format_override
        .map(|f| f.trim_start_matches('.').to_ascii_lowercase())
        .or_else(|| path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()))
        .ok_or_else(|| AutomationError::BadRequest(format!("cannot tell the format of `{}`; pass a format", path.display())))?;
    if ext == sary_format::EXTENSION {
        let named = named_for_output(doc, &path.to_string_lossy());
        let mut local = SaryWriter::new();
        let w = writer.unwrap_or(&mut local);
        w.save_path(&named, path, &previews(&named))?;
        return Ok(Vec::new());
    }
    let r = sary_io::export(doc, &ext, opts)?;
    // Crash-safe: a failed write never destroys the previous file.
    sary_format::atomic_write(path, &r.bytes).map_err(|e| AutomationError::Io(e.to_string()))?;
    Ok(r.warnings)
}

/// Encode a document for a capability-scoped write. Unlike [`save`], this
/// never obtains ambient filesystem authority and emits `.sary` as a ZIP.
pub fn save_bytes(doc: &Document, name: &str, format_override: Option<&str>, opts: &ExportOptions) -> Result<(Vec<u8>, Vec<String>), AutomationError> {
    let ext = format_override
        .map(|format| format.trim_start_matches('.').to_ascii_lowercase())
        .or_else(|| Path::new(name).extension().map(|extension| extension.to_string_lossy().to_ascii_lowercase()))
        .ok_or_else(|| AutomationError::BadRequest(format!("cannot tell the format of `{name}`; pass a format")))?;
    if ext == sary_format::EXTENSION {
        let named = named_for_output(doc, name);
        return Ok((sary_format::save_to_bytes(&named, &previews(&named))?, Vec::new()));
    }
    let result = sary_io::export(doc, &ext, opts)?;
    Ok((result.bytes, result.warnings))
}

/// Flattened document as PNG, scaled to fit `max_side` (0 = full size).
pub fn render_png(doc: &Document, max_side: u32) -> Result<Vec<u8>, AutomationError> {
    let side = if max_side == 0 { doc.size.width.max(doc.size.height) } else { max_side };
    let img = sary_compose::thumbnail(doc, side.max(1));
    let image = sary_codecs::Image::from_u8(img.width, img.height, sary_codecs::ChannelLayout::Rgba, img.pixels)
        .map_err(|e| AutomationError::Other(e.to_string()))?;
    sary_codecs::encode(&image, sary_codecs::Format::Png, &Default::default()).map_err(|e| AutomationError::Other(e.to_string()))
}
