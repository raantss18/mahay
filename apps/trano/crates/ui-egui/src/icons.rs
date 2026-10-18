//! Bundled images: each Mahay App's icon and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a Mahay App.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "sary" => egui::include_image!("../../../assets/app-icons/sary.png"),
        "tsipika" => egui::include_image!("../../../assets/app-icons/tsipika.png"),
        "hetsika" => egui::include_image!("../../../assets/app-icons/hetsika.png"),
        "hazavana" => egui::include_image!("../../../assets/app-icons/hazavana.png"),
        "taratasy" => egui::include_image!("../../../assets/app-icons/taratasy.png"),
        "tselatra" => egui::include_image!("../../../assets/app-icons/tselatra.png"),
        "pejy" => egui::include_image!("../../../assets/app-icons/pejy.png"),
        "soratra" => egui::include_image!("../../../assets/app-icons/soratra.png"),
        "seho" => egui::include_image!("../../../assets/app-icons/seho.png"),
        "tabilao" => egui::include_image!("../../../assets/app-icons/tabilao.png"),
        "drafitra" => egui::include_image!("../../../assets/app-icons/drafitra.png"),
        "feo" => egui::include_image!("../../../assets/app-icons/feo.png"),
        "endrika" => egui::include_image!("../../../assets/app-icons/endrika.png"),
        _ => return None,
    })
}

/// The Mahay wordmark, as supplied: dark ink for light backgrounds, white for dark ones.
/// Mahay artwork, MIT OR Apache-2.0 like the code (`docs/brand/LICENSE-brand.txt`).
pub fn mahay_logo(dark_background: bool) -> ImageSource<'static> {
    if dark_background {
        egui::include_image!("../../../docs/brand/mahay-logo-white.svg")
    } else {
        egui::include_image!("../../../docs/brand/mahay-logo.svg")
    }
}

/// The Mahay wordmark's width / height (its SVG view box: 400 × 108).
pub const MAHAY_LOGO_ASPECT: f32 = 400.0 / 108.0;

/// A social service's logo, by its name in `catalog::SOCIAL`. Mahay lists no social accounts
/// yet, so there are none.
pub fn social(_service: &str) -> Option<ImageSource<'static>> {
    None
}

/// The Mahay mark: the launcher's app icon.
pub fn mahay_mark() -> ImageSource<'static> {
    egui::include_image!("../../../docs/brand/mahay-mark.svg")
}
