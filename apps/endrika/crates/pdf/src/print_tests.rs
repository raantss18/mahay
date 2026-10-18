//! Print options: page boxes, the sheet grown by the bleed and marks, the marks themselves, what
//! shows in the bleed, CMYK colour everywhere colour is written, and hostile settings.

use super::*;
use crate::print::{MARK_GAP, MARK_LENGTH, cmyk_of};
use crate::tests::{inflated, sample_doc};
use figcraft_color::{Color, ColorStop, Gradient, GradientKind, Paint};
use figcraft_doc::{Effect, EffectKind, NodeKind};

fn text(pdf: &[u8]) -> String {
    String::from_utf8_lossy(pdf).into_owned()
}

/// A page box's four numbers (`None` without one).
fn pdf_box(pdf: &[u8], name: &str) -> Option<[f64; 4]> {
    let s = text(pdf);
    let rest = s.get(s.find(&format!("/{name}["))? + name.len() + 2..)?;
    let nums: Vec<f64> = rest.get(..rest.find(']')?)?.split_whitespace().filter_map(|v| v.parse().ok()).collect();
    nums.try_into().ok()
}

/// A 200 × 100 red frame.
fn frame() -> (Document, NodeId) {
    let mut d = Document::new("print");
    let p = d.first_page().unwrap();
    let f = d.create(p, NodeKind::Frame, "Poster", 0.0, 0.0, 200.0, 100.0).unwrap();
    d.get_mut(f).unwrap().fills = vec![Paint::solid(Color::from_hex("FF0000").unwrap())];
    (d, f)
}

fn export(d: &Document, f: NodeId, print: PrintSettings) -> Vec<u8> {
    export_pdf(d, &[f], &PdfOptions { print, ..Default::default() }).unwrap()
}

#[test]
fn page_boxes_frame_the_trim_bleed_and_sheet() {
    let (d, f) = frame();
    // Plain: the page is the frame, as before.
    let plain = export(&d, f, PrintSettings::default());
    assert_eq!(pdf_box(&plain, "MediaBox"), Some([0.0, 0.0, 200.0, 100.0]));
    assert_eq!((pdf_box(&plain, "BleedBox"), pdf_box(&plain, "TrimBox")), (None, None));
    // A bleed grows the sheet by it.
    let bled = export(&d, f, PrintSettings { bleed: 9.0, ..Default::default() });
    assert_eq!(pdf_box(&bled, "MediaBox"), Some([0.0, 0.0, 218.0, 118.0]));
    assert_eq!(pdf_box(&bled, "BleedBox"), Some([0.0, 0.0, 218.0, 118.0]));
    assert_eq!(pdf_box(&bled, "TrimBox"), Some([9.0, 9.0, 209.0, 109.0]));
    // Marks need room beyond the bleed.
    let m = 9.0 + 2.0 * MARK_GAP + MARK_LENGTH;
    let marked = export(&d, f, PrintSettings { bleed: 9.0, crop_marks: true, ..Default::default() });
    assert_eq!(pdf_box(&marked, "MediaBox"), Some([0.0, 0.0, 200.0 + 2.0 * m, 100.0 + 2.0 * m]));
    assert_eq!(pdf_box(&marked, "BleedBox"), Some([m - 9.0, m - 9.0, m + 209.0, m + 109.0]));
    assert_eq!(pdf_box(&marked, "TrimBox"), Some([m, m, m + 200.0, m + 100.0]));
    // Crop marks without a bleed: the trim is the bleed.
    let crop = export(&d, f, PrintSettings { crop_marks: true, ..Default::default() });
    assert_eq!(pdf_box(&crop, "TrimBox"), pdf_box(&crop, "BleedBox"));
}

#[test]
fn a_frame_trims_to_its_box_not_its_shadow() {
    let (mut d, f) = frame();
    d.get_mut(f).unwrap().effects = vec![Effect { kind: EffectKind::DropShadow(Default::default()), visible: true }];
    // Plain export takes the shadow in; for print the shadow goes into the bleed.
    let plain = export(&d, f, PrintSettings::default());
    assert!(pdf_box(&plain, "MediaBox").is_some_and(|b| b[3] > 100.0));
    let print = export(&d, f, PrintSettings { bleed: 9.0, ..Default::default() });
    assert_eq!(pdf_box(&print, "TrimBox"), Some([9.0, 9.0, 209.0, 109.0]));
}

#[test]
fn crop_and_bleed_marks_are_hairlines_outside_the_bleed() {
    let (d, f) = frame();
    let marks = |p: PrintSettings| {
        let s = inflated(&export(&d, f, p));
        let block = s.get(s.find(" w ").unwrap_or(0)..).unwrap_or("").to_string();
        (s.contains("0.25 w"), block.matches(" m ").count(), s)
    };
    let (none, ..) = marks(PrintSettings { bleed: 9.0, ..Default::default() });
    assert!(!none, "no marks asked for");
    // Four corners × two lines.
    let (hair, lines, s) = marks(PrintSettings { bleed: 9.0, crop_marks: true, ..Default::default() });
    assert!(hair && lines == 8, "{s}");
    // The top-left crop marks: along the trim's top and left edges, from just outside the bleed.
    let m = 9.0 + 2.0 * MARK_GAP + MARK_LENGTH;
    let (near, far) = (m - 9.0 - MARK_GAP, m - 9.0 - MARK_GAP - MARK_LENGTH);
    assert!(s.contains(&format!("{near} {m} m {far} {m} l")) && s.contains(&format!("{m} {near} m {m} {far} l")), "{s}");
    // In registration black (RGB) or all four inks (CMYK).
    assert!(s.contains("0 0 0 RG"), "{s}");
    let (_, lines, s) = marks(PrintSettings { bleed: 9.0, crop_marks: true, bleed_marks: true, cmyk: true });
    assert!(lines == 16 && s.contains("1 1 1 1 K"), "{s}");
    // Bleed marks: along the bleed's edges.
    let edge = m - 9.0;
    assert!(s.contains(&format!("{near} {edge} m {far} {edge} l")), "{s}");
    // Bleed marks without a bleed would sit on the crop marks: none.
    let (hair, ..) = marks(PrintSettings { bleed_marks: true, ..Default::default() });
    assert!(!hair);
}

#[test]
fn the_bleed_shows_what_lies_past_the_frame() {
    let mut d = Document::new("bleed");
    let p = d.first_page().unwrap();
    let f = d.create(p, NodeKind::Frame, "F", 0.0, 0.0, 100.0, 100.0).unwrap();
    d.get_mut(f).unwrap().fills = vec![Paint::solid(Color::from_hex("0000FF").unwrap())];
    // A band running past both sides of the frame.
    let band = d.create(f, NodeKind::Rectangle, "Band", -20.0, 40.0, 140.0, 20.0).unwrap();
    d.get_mut(band).unwrap().fills = vec![Paint::solid(Color::from_hex("FF0000").unwrap())];
    let s = inflated(&export(&d, f, PrintSettings::default()));
    // Without a bleed: the frame's fill and clip are its box.
    assert!(s.contains("0 0 m 100 0 l 100 100 l 0 100 l") && s.contains("0 100 m 100 100 l 100 0 l 0 0 l"), "{s}");
    let s = inflated(&export(&d, f, PrintSettings { bleed: 10.0, ..Default::default() }));
    // With one: the fill reaches out 10 on every side (in the frame's space) ...
    assert!(s.contains("0 0 1 rg -10 -10 m 110 -10 l 110 110 l -10 110 l"), "{s}");
    // ... and the band is clipped to the bleed (the 120-point sheet), not the frame.
    assert_eq!(s.matches("0 120 m 120 120 l 120 0 l 0 0 l").count(), 2, "{s}");
    assert!(!s.contains("10 110 m 110 110 l"), "no clip at the trim: {s}");
    assert!(s.contains("1 0 0 rg 0 0 m 140 0 l"), "the band is drawn whole: {s}");
}

#[test]
fn layers_that_arent_frames_get_a_bleed_around_their_bounds() {
    let mut d = Document::new("nodes");
    let p = d.first_page().unwrap();
    let r = d.create(p, NodeKind::Rectangle, "R", 0.0, 0.0, 50.0, 50.0).unwrap();
    let pdf = export_groups(&d, &[vec![r]], &PdfOptions { print: PrintSettings { bleed: 5.0, ..Default::default() }, ..Default::default() }).unwrap();
    assert_eq!(pdf_box(&pdf, "TrimBox"), Some([5.0, 5.0, 55.0, 55.0]));
    assert_eq!(pdf_box(&pdf, "MediaBox"), Some([0.0, 0.0, 60.0, 60.0]));
}

#[test]
fn cmyk_conversion_is_the_documented_formula() {
    let near = |a: [f32; 4], b: [f32; 4]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-6);
    assert!(near(cmyk_of(1.0, 0.0, 0.0), [0.0, 1.0, 1.0, 0.0]));
    assert!(near(cmyk_of(1.0, 1.0, 1.0), [0.0, 0.0, 0.0, 0.0]));
    assert!(near(cmyk_of(0.0, 0.0, 0.0), [0.0, 0.0, 0.0, 1.0]));
    assert!(near(cmyk_of(0.5, 0.5, 0.5), [0.0, 0.0, 0.0, 0.5]));
    assert!(near(cmyk_of(0.2, 0.4, 0.8), [0.75, 0.5, 0.0, 0.2]));
    // Out of range and not-a-number channels count as their nearest valid value (NaN as 0).
    assert!(near(cmyk_of(f32::NAN, 2.0, -1.0), cmyk_of(0.0, 1.0, 0.0)));
}

#[test]
fn cmyk_writes_every_colour_in_cmyk() {
    let (mut d, f) = frame();
    // A gradient, an image, a shadow (rasterised) and a stroke besides the solid fill.
    let r = d.create(f, NodeKind::Rectangle, "G", 10.0, 10.0, 80.0, 40.0).unwrap();
    {
        let n = d.get_mut(r).unwrap();
        let stops = vec![ColorStop { position: 0.0, color: Color::from_hex("00FF00").unwrap() }, ColorStop { position: 1.0, color: Color::WHITE }];
        n.fills = vec![Paint::gradient(Gradient { kind: GradientKind::Linear, stops, from: (0.0, 0.5), to: (1.0, 0.5), width: (0.0, 1.0) })];
        n.strokes = vec![Paint::solid(Color::from_hex("336699").unwrap())];
        n.stroke.weight = 2.0;
        n.effects = vec![Effect { kind: EffectKind::DropShadow(Default::default()), visible: true }];
    }
    let rgb = export(&d, f, PrintSettings::default());
    let cmyk = export(&d, f, PrintSettings { cmyk: true, ..Default::default() });
    let (rs, cs) = (inflated(&rgb), inflated(&cmyk));
    assert!(rs.contains("1 0 0 rg") && !rs.contains(" k "), "{rs}");
    // Red is magenta and yellow; no RGB colour operators are left.
    assert!(cs.contains("0 1 1 0 k"), "{cs}");
    assert!(!cs.contains(" rg") && !cs.contains(" RG"), "{cs}");
    // The gradient's shading and the shadow image are CMYK too.
    let t = text(&cmyk);
    assert!(t.contains("/DeviceCMYK"), "{t}");
    assert!(!t.contains("/ColorSpace/DeviceRGB"), "{t}");
    // The same file every time.
    assert_eq!(cmyk, export(&d, f, PrintSettings { cmyk: true, ..Default::default() }));
}

#[test]
fn cmyk_images_and_the_sample_document() {
    let (d, frames) = sample_doc();
    let pdf = export_pdf(
        &d,
        &frames,
        &PdfOptions { print: PrintSettings { cmyk: true, bleed: 8.0, crop_marks: true, ..Default::default() }, ..Default::default() },
    )
    .unwrap();
    let t = text(&pdf);
    assert_eq!(t.matches("/TrimBox[").count(), frames.len());
    // Images (the image fill, rasterised effects) are CMYK, with grey soft masks for alpha.
    assert!(t.contains("/Subtype/Image") && !t.contains("/ColorSpace/DeviceRGB"), "{}", t.chars().take(4000).collect::<String>());
    assert!(inflated(&pdf).contains(" k "), "CMYK fills");
}

#[test]
fn soft_masks_stay_rgb_while_what_they_mask_prints_in_cmyk() {
    let (mut d, f) = frame();
    let g = d.create(f, NodeKind::Group, "G", 0.0, 0.0, 100.0, 100.0).unwrap();
    let m = d.create(g, NodeKind::Rectangle, "Mask", 0.0, 0.0, 100.0, 100.0).unwrap();
    {
        let n = d.get_mut(m).unwrap();
        n.is_mask = true;
        n.mask_type = figcraft_doc::MaskType::Luminance;
        n.fills = vec![Paint::solid(Color::from_hex("808080").unwrap())];
    }
    let c = d.create(g, NodeKind::Rectangle, "Content", 0.0, 0.0, 100.0, 100.0).unwrap();
    d.get_mut(c).unwrap().fills = vec![Paint::solid(Color::from_hex("0000FF").unwrap())];
    let s = inflated(&export(&d, f, PrintSettings { cmyk: true, ..Default::default() }));
    // The mask's grey (never printed) and the content's blue as cyan and magenta.
    assert!(s.contains("0.5019608 0.5019608 0.5019608 rg") && s.contains("1 1 0 0 k"), "{s}");
}

#[test]
fn hostile_print_settings_still_export() {
    let (d, f) = frame();
    for print in [
        PrintSettings { bleed: f64::NAN, crop_marks: true, bleed_marks: true, cmyk: true },
        PrintSettings { bleed: f64::INFINITY, bleed_marks: true, ..Default::default() },
        PrintSettings { bleed: -50.0, crop_marks: true, ..Default::default() },
        PrintSettings { bleed: 1e12, crop_marks: true, ..Default::default() },
    ] {
        let pdf = export(&d, f, print);
        let media = pdf_box(&pdf, "MediaBox").unwrap();
        assert!(
            media.iter().all(|v| v.is_finite()) && media[2] <= 200.0 + 2.0 * (PrintSettings::MAX_BLEED + 2.0 * MARK_GAP + MARK_LENGTH) + 1e-6,
            "{print:?}: {media:?}"
        );
    }
    // A frame too big for a page is refused, with or without a bleed.
    let mut d = Document::new("big");
    let p = d.first_page().unwrap();
    let big = d.create(p, NodeKind::Frame, "Big", 0.0, 0.0, MAX_PAGE_SIDE - 10.0, 10.0).unwrap();
    assert!(export_pdf(&d, &[big], &PdfOptions::default()).is_ok());
    assert!(matches!(
        export_pdf(&d, &[big], &PdfOptions { print: PrintSettings { bleed: 20.0, ..Default::default() }, ..Default::default() }),
        Err(PdfError::TooLarge(..))
    ));
}
