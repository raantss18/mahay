//! The Mahay App Suite: the thirteen apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MahayApp {
    /// Machine name: repo, binary, package and asset stem (`sary`).
    pub id: &'static str,
    /// Display name (`Sary`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: mahay.com's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (Taratasy shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (Taratasy's MSI said PrintCraft until 0.2.1).
    pub former_names: &'static [&'static str],
}

impl MahayApp {
    /// `org.mahay.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("org.mahay.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page on mahay.com.
    pub fn website(&self) -> String {
        format!("https://github.com/raantss18/mahay/tree/main/apps/{}", self.id)
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static MahayApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> MahayApp {
    MahayApp { id, name, tagline, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// The Mahay App Suite, in display order matching mahay.com:
pub const SUITE: [MahayApp; 13] = [
    app("sary", "Sary", "Image editing", [[0x2e, 0x86, 0xde], [0x24, 0x6b, 0xb1], [0x74, 0xcc, 0xff]], "raantss18/mahay"),
    app("tsilo", "Tsilo", "Vector illustration", [[0x8e, 0x44, 0xad], [0x71, 0x36, 0x8a], [0xd4, 0x8a, 0xf3]], "raantss18/mahay"),
    app(
        "hetsika",
        "Hetsika",
        "Video editing, color and sound",
        [[0x37, 0x47, 0x4f], [0x2c, 0x38, 0x3f], [0x7d, 0x8d, 0x95]],
        "raantss18/mahay",
    ),
    app(
        "hazavana",
        "Hazavana",
        "Photo library and raw development",
        [[0xe0, 0x9f, 0x00], [0xb3, 0x7f, 0x00], [0xff, 0xe5, 0x46]],
        "raantss18/mahay",
    ),
    MahayApp {
        former_ids: &["printcraft"],
        former_names: &["PrintCraft"],
        ..app(
            "taratasy",
            "Taratasy",
            "Reading, organizing and protecting PDFs",
            [[0xd7, 0x26, 0x3d], [0xac, 0x1e, 0x30], [0xff, 0x6c, 0x83]],
            "raantss18/mahay",
        )
    },
    app(
        "tselatra",
        "Tselatra",
        "Motion graphics and visual effects",
        [[0x7c, 0xb3, 0x42], [0x63, 0x8f, 0x34], [0xc2, 0xf9, 0x88]],
        "raantss18/mahay",
    ),
    app(
        "pejy",
        "Pejy",
        "Page layout and publishing",
        [[0x79, 0x55, 0x48], [0x60, 0x44, 0x39], [0xbf, 0x9b, 0x8e]],
        "raantss18/mahay",
    ),
    app(
        "endrika",
        "Endrika",
        "Interface design and prototyping",
        [[0x00, 0x97, 0xa7], [0x00, 0x78, 0x85], [0x46, 0xdd, 0xed]],
        "raantss18/mahay",
    ),
    app("soratra", "Soratra", "Writing and document design", [[0x3f, 0x51, 0xb5], [0x32, 0x40, 0x90], [0x85, 0x97, 0xfb]], "raantss18/mahay"),
    app(
        "feo",
        "Feo",
        "Recording, editing and mixing audio",
        [[0xd8, 0x1b, 0x60], [0xac, 0x15, 0x4c], [0xff, 0x61, 0xa6]],
        "raantss18/mahay",
    ),
    app(
        "seho",
        "Seho",
        "Presentations and slide shows",
        [[0xf2, 0x8f, 0x3b], [0xc1, 0x72, 0x2f], [0xff, 0xd5, 0x81]],
        "raantss18/mahay",
    ),
    app(
        "drafitra",
        "Drafitra",
        "Computer-aided design and drafting",
        [[0x0d, 0x47, 0xa1], [0x0a, 0x38, 0x80], [0x53, 0x8d, 0xe7]],
        "raantss18/mahay",
    ),
    app("tabilao", "Tabilao", "Spreadsheets and data", [[0x1b, 0x99, 0x8b], [0x15, 0x7a, 0x6f], [0x61, 0xdf, 0xd1]], "raantss18/mahay"),
];

/// Mahay's website.
pub const WEBSITE_URL: &str = "https://github.com/raantss18/mahay";
/// The GitHub account every Mahay App's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
pub const GITHUB_OWNER: &str = "raantss18";
/// Mahay on GitHub: every Mahay App's source.
pub const GITHUB_URL: &str = "https://github.com/raantss18";
/// Mahay's social accounts: (service, URL). Shown as small icons, less prominent than Discord.
/// Mahay's social accounts: (service, URL). None yet; upstream's accounts are not Mahay's.
pub const SOCIAL: &[(&str, &str)] = &[];

/// Where the Mahay community talks: GitHub Discussions (Mahay has no Discord server).
pub const DISCORD_URL: &str = "https://github.com/raantss18/mahay/discussions";
/// The same place; kept for the "open in the app" path, which falls back to the URL.
pub const DISCORD_APP_URL: &str = "https://github.com/raantss18/mahay/discussions";

/// Mahay Trano itself, described like a Mahay App so it can update itself with the same
/// machinery (asset names `trano-<ver>-…`, bundle id `org.mahay.trano`,
/// `Mahay Trano.app`). Not part of [`SUITE`].
pub const LAUNCHER: MahayApp = app(
    "trano",
    "Mahay Trano",
    "Installs and updates the Mahay App Suite",
    [[0x0b, 0x0b, 0x0c], [0x0b, 0x0b, 0x0c], [0xf2, 0xf1, 0xee]],
    "raantss18/mahay",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "raantss18/mahay";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "trano";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_has_thirteen_apps_with_unique_ids() {
        assert_eq!(SUITE.len(), 13);
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, "raantss18/mahay");
            assert_eq!(a.name.to_lowercase(), a.id);
        }
    }

    #[test]
    fn taratasy_also_answers_to_printcraft() {
        let pdf = MahayApp::by_id("taratasy").unwrap();
        assert_eq!(pdf.stems().collect::<Vec<_>>(), ["taratasy", "printcraft"]);
        assert_eq!(pdf.names().collect::<Vec<_>>(), ["Taratasy", "PrintCraft"]);
        assert_eq!(pdf.website(), "https://github.com/raantss18/mahay");
        assert_eq!(MahayApp::by_id("soratra").unwrap().website(), "https://github.com/raantss18/mahay");
    }
}
