//! The About page and the update banner in English, French and Malagasy.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Lang {
    #[default]
    En,
    Fr,
    Mg,
}

impl Lang {
    /// From a BCP 47 tag or a POSIX locale (`fr-FR`, `mg_MG.UTF-8`, `en`).
    pub fn from_tag(tag: &str) -> Lang {
        match tag.trim().split(['-', '_', '.']).next().unwrap_or_default().to_ascii_lowercase().as_str() {
            "fr" => Lang::Fr,
            "mg" => Lang::Mg,
            _ => Lang::En,
        }
    }

    /// From the environment (`LC_ALL`, `LC_MESSAGES`, `LANG`) on desktop.
    pub fn from_env() -> Lang {
        ["LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .find_map(|k| std::env::var(k).ok().filter(|v| !v.is_empty() && v != "C" && v != "POSIX"))
            .map(|v| Lang::from_tag(&v))
            .unwrap_or_default()
    }
}

/// The text for `key` (the English text when a key is unknown).
pub fn t(lang: Lang, key: &str) -> &'static str {
    let row: Option<(&str, &str, &str)> = match key {
        "suite" => Some(("Part of the Mahay suite: free apps that work offline", "Fait partie de la suite Mahay : des apps libres qui marchent hors ligne", "Ao anatin'ny andiany Mahay: rindranasa malalaka tsy mila aterineto")),
        "updates" => Some(("Updates", "Mises à jour", "Fanavaozana")),
        "auto" => Some(("Update automatically (check every day)", "Mettre à jour automatiquement (vérifier chaque jour)", "Havaozina ho azy (jerena isan'andro)")),
        "installed" => Some(("Installed version", "Version installée", "Dikan-teny napetraka")),
        "checking" => Some(("Checking for updates…", "Recherche de mises à jour…", "Mitady fanavaozana…")),
        "up_to_date" => Some(("Mahay is up to date", "Mahay est à jour", "Vaovao indrindra i Mahay")),
        "available" => Some(("A new version is available:", "Une nouvelle version est disponible :", "Misy dikan-teny vaovao:")),
        "downloading" => Some(("Downloading", "Téléchargement de la version", "Maka ny dikan-teny")),
        "ready" => Some(("Ready to install:", "Prête à installer :", "Vonona hapetraka:")),
        "ready_short" => Some(("update ready", "mise à jour prête", "fanavaozana vonona")),
        "whats_new" => Some(("What's new", "Nouveautés", "Ny vaovao")),
        "install" => Some(("Install", "Installer", "Apetraho")),
        "later" => Some(("Later", "Plus tard", "Rahatrizay")),
        "failed" => Some(("Update failed", "Échec de la mise à jour", "Tsy nety ny fanavaozana")),
        "check_now" => Some(("Check now", "Vérifier maintenant", "Jereo izao")),
        "qr_title" => Some(("Install Mahay on another device", "Installer Mahay sur un autre appareil", "Apetraho amin'ny fitaovana hafa i Mahay")),
        "qr_help" => Some(("Scan with the other phone's camera, then install Mahay HUB: it installs the other apps.", "Scannez avec l'appareil photo de l'autre téléphone, puis installez Mahay HUB : il installe les autres apps.", "Scannez amin'ny fakantsarin'ilay finday hafa, dia apetraho i Mahay HUB: izy no mametraka ny hafa.")),
        "qr_link" => Some(("Latest version", "Dernière version", "Dikan-teny farany")),
        "author" => Some(("Author", "Auteur", "Mpanoratra")),
        "author_role" => Some(("Design and development", "Conception et développement", "Famolavolana sy fampandrosoana")),
        "discussions" => Some(("Questions and ideas: discussions", "Questions et idées : discussions", "Fanontaniana sy hevitra: dinika")),
        "built_with" => Some(("Built with", "Construit avec", "Natao tamin'ny")),
        "license" => Some(("Free software: MIT OR Apache-2.0.", "Logiciel libre : MIT OR Apache-2.0.", "Rindrankajy malalaka: MIT OR Apache-2.0.")),
        "third_party" => Some(("Third-party licences", "Licences tierces", "Fahazoan-dalana hafa")),
        "tag_hub" => Some(("The Mahay hub: install, open and update every app", "L'accueil Mahay : installer, ouvrir et mettre à jour toutes les apps", "Fandraisana Mahay: mametraka, manokatra ary manavao ny rindranasa rehetra")),
        "tag_sary" => Some(("Image editor", "Retouche d'images", "Fanamboarana sary")),
        "tag_taratasy" => Some(("PDF: read, annotate, fill, sign", "PDF : lire, annoter, remplir, signer", "PDF: vakiana, asiana fanamarihana, fenoina, soniavina")),
        "tag_soratra" => Some(("Word processor", "Traitement de texte", "Fanoratana lahatsoratra")),
        "tag_tabilao" => Some(("Spreadsheet", "Tableur", "Tabilao kajy")),
        "tag_seho" => Some(("Presentations", "Présentations", "Fampisehoana")),
        "tag_tsilo" => Some(("Vector illustration", "Illustration vectorielle", "Sary vektoraly")),
        "tag_endrika" => Some(("Interface design and prototyping", "Design d'interfaces et prototypes", "Famolavolana endrika sy modely")),
        "tag_pejy" => Some(("Page layout and publishing", "Mise en page et publication", "Fandaminana pejy sy famoahana")),
        "tag_hazavana" => Some(("Photo library and raw development", "Photothèque et développement RAW", "Tahirin-tsary sy RAW")),
        "tag_feo" => Some(("Recording, editing and mixing audio", "Enregistrement, montage et mixage audio", "Fandraisam-peo, fanamboarana ary fampifangaroana")),
        "tag_hetsika" => Some(("Video editing", "Montage vidéo", "Fanamboarana horonantsary")),
        "tag_tselatra" => Some(("Motion graphics and visual effects", "Animation et effets visuels", "Sary mihetsika sy effets")),
        "tag_drafitra" => Some(("Technical drawing (CAD)", "Dessin technique (CAO)", "Sary teknika (CAO)")),
        _ => None,
    };
    match (row, lang) {
        (Some((en, _, _)), Lang::En) => en,
        (Some((_, fr, _)), Lang::Fr) => fr,
        (Some((_, _, mg)), Lang::Mg) => mg,
        (None, _) => "",
    }
}
