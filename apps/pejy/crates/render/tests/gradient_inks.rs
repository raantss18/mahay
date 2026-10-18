//! A gradient between process colours shows the inks' blend, as InDesign shows it.
use pejy_color::swatch::Swatch;
use pejy_color::{Color, Gradient, GradientKind, GradientStop, SwatchValue};
use pejy_compose::Cache;
use pejy_doc::build::NewDocument;
use pejy_doc::{Document, Fill, Item, ItemId, Shape, SpreadRef, Stroke};
use pejy_geom::{Affine, Rect, shapes};
use pejy_render::{Placed, RenderOptions, Renderer};

#[test]
fn a_cmyk_gradient_is_c50_m50_halfway() {
    let mut d = Document::new(&NewDocument { width: 100.0, height: 20.0, facing_pages: false, ..Default::default() });
    let stop = |offset: f32, color: Color| GradientStop { offset, color, opacity: 1.0, midpoint: 0.5 };
    let gradient =
        Gradient { kind: GradientKind::Linear, stops: vec![stop(0.0, Color::cmyk(0.0, 1.0, 0.0, 0.0)), stop(1.0, Color::cmyk(1.0, 0.0, 0.0, 0.0))] };
    d.swatches.push(Swatch { name: "Magenta to Cyan".into(), value: SwatchValue::Gradient { gradient }, locked: false, named: true, hidden: false });
    let id = ItemId(d.alloc());
    let mut item = Item::new(id, d.default_layer(), Shape::Rectangle, shapes::rectangle(Rect::new(0.0, 0.0, 100.0, 20.0)));
    item.fill = Fill::swatch("Magenta to Cyan");
    item.stroke = Stroke::default();
    d.insert_item(SpreadRef::Doc(0), item, None).unwrap();
    let img = Renderer::new().render(
        &d,
        &Cache::new(),
        &[Placed { spread: SpreadRef::Doc(0), xf: Affine::IDENTITY }],
        100,
        20,
        Affine::IDENTITY,
        &RenderOptions { paper: false, background: Some([255; 4]), ..Default::default() },
    );
    let want = Color::cmyk(0.5, 0.5, 0.0, 0.0).to_rgba8(1.0);
    let got = img.pixel(50, 10);
    for ch in 0..3 {
        assert!(got[ch].abs_diff(want[ch]) <= 4, "halfway {got:?}, the inks' blend is {want:?}");
    }
}
