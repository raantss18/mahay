//! The angle settings round-trip through DXF: `$ANGBASE` (group 50) in radians, `$ANGDIR` and
//! `$AUNITS` (group 70) as integers.

use cadcraft_doc::{Drawing, HVal};

#[test]
fn angbase_angdir_aunits_round_trip_through_dxf() {
    let mut d = Drawing::new_metric();
    d.header.set_f64("ANGBASE", std::f64::consts::FRAC_PI_2);
    d.header.set_i64("ANGDIR", 1);
    d.header.set_i64("AUNITS", 3);
    let text = cadcraft_io::write_dxf(&d);
    let r = cadcraft_io::read_dxf(text.as_bytes()).unwrap();
    let base = r.header.f64("ANGBASE", 0.0);
    assert!((base - std::f64::consts::FRAC_PI_2).abs() < 1e-12, "$ANGBASE read back as {base}");
    assert_eq!(r.header.get("ANGDIR"), Some(&HVal::Int(1)));
    assert_eq!(r.header.get("AUNITS"), Some(&HVal::Int(3)));
}
