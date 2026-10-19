//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "Tselatra";
pub const APP_SLUG: &str = "tselatra";
pub const DISCORD: &str = "https://github.com/raantss18/mahay/discussions";
pub const WEBSITE: &str = "https://github.com/raantss18/mahay";
pub const APP_PAGE: &str = "https://github.com/raantss18/mahay";
pub const GITHUB: &str = "https://github.com/raantss18/mahay";
pub const ISSUES: &str = "https://github.com/raantss18/mahay/issues";

/// Sibling apps of the Mahay family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("Sary", "sary"),
    ("Tsilo", "tsilo"),
    ("Hetsika", "hetsika"),
    ("Hazavana", "hazavana"),
    ("Taratasy", "taratasy"),
    ("Pejy", "pejy"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/raantss18/mahay/tree/main/apps/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://github.com/raantss18/mahay/tree/main/apps/{slug}")
}
