//! Pattern cells: grid and half-shifted rows or columns, spacing, alignment, and sanitised values.

use crate::{PatternAlign, PatternPaint, PatternTile};

#[test]
fn cells_tile_grids_and_shifted_rows() {
    let mut p = PatternPaint::new(7);
    p.spacing = (5.0, 10.0);
    let c = p.cell(20.0, 10.0, 100.0, 100.0).unwrap();
    assert_eq!((c.size, c.origin, c.tiles.len()), ((25.0, 20.0), (0.0, 0.0), 1));
    p.tile = PatternTile::HorizontalHex;
    let c = p.cell(20.0, 10.0, 100.0, 100.0).unwrap();
    assert_eq!(c.size, (25.0, 40.0));
    assert_eq!(c.tiles, vec![(0.0, 0.0), (12.5, 20.0), (-12.5, 20.0)]);
    p.tile = PatternTile::VerticalHex;
    assert_eq!(p.cell(20.0, 10.0, 100.0, 100.0).unwrap().tiles, vec![(0.0, 0.0), (25.0, 10.0), (25.0, -10.0)]);
    // Scale applies to the tile, not the gap; alignment puts a tile in the centre or the far corner.
    p.tile = PatternTile::Grid;
    p.scale = 2.0;
    p.align = PatternAlign::Center;
    let c = p.cell(20.0, 10.0, 100.0, 100.0).unwrap();
    assert_eq!((c.size, c.origin, c.scale), ((45.0, 30.0), (30.0, 40.0), 2.0));
    p.align = PatternAlign::End;
    assert_eq!(p.cell(20.0, 10.0, 100.0, 100.0).unwrap().origin, (60.0, 80.0));
}

#[test]
fn cells_sanitise_their_values() {
    let mut p = PatternPaint::new(1);
    assert!(p.cell(0.0, 10.0, 10.0, 10.0).is_none(), "a source without size");
    p.scale = f64::NAN;
    p.spacing = (f64::INFINITY, -5.0);
    let c = p.cell(10.0, 10.0, 10.0, 10.0).unwrap();
    assert_eq!((c.scale, c.size), (1.0, (10.0, 10.0)));
    p.scale = 1e9;
    p.spacing = (1e9, 0.0);
    let c = p.cell(10.0, 10.0, 10.0, 10.0).unwrap();
    assert_eq!((c.scale, c.size.0), (PatternPaint::SCALE_RANGE.1, 1000.0 + PatternPaint::MAX_SPACING));
    // Old files and hand-written paints: only the source is required.
    let p: crate::Paint = serde_json::from_str(r#"{"type":"pattern","source":3}"#).unwrap();
    assert_eq!(p.kind, crate::PaintKind::Pattern(PatternPaint::new(3)));
}
