//! Gradient paints from agents get a linear width handle square to their axis (a missing one was
//! put on the side's middle: the bands skewed, and vanished for a left-to-right gradient).

use serde_json::{Map, Value, json};

use crate::tools::normalize_props;

fn width_of(paint: Value) -> Value {
    let mut props: Map<String, Value> = json!({"fills": [paint]}).as_object().cloned().unwrap();
    normalize_props(&mut props);
    props["fills"][0]["width"].clone()
}

#[test]
fn a_linear_gradient_without_a_width_handle_gets_a_square_one() {
    assert_eq!(width_of(json!({"type": "linearGradient", "stops": ["#000000", "#FFFFFF"]})), json!([-0.5, 0.0]));
    assert_eq!(width_of(json!({"type": "gradient", "from": [0.0, 0.5], "to": [1.0, 0.5], "stops": ["#000000", "#FFFFFF"]})), json!([0.0, 1.5]));
    // Given handles and other kinds are left alone.
    assert_eq!(width_of(json!({"type": "gradient", "from": [0.0, 0.5], "to": [1.0, 0.5], "width": [0.0, 1.0]})), json!([0.0, 1.0]));
    assert_eq!(width_of(json!({"type": "radialGradient"})), json!([1.0, 0.5]));
}
