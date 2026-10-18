//! Prints the app-level parity summary and the menu items not yet implemented.

fn main() {
    let v = feo_ui_egui::menus::parity();
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
}
