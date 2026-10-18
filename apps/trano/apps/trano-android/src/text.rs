//! The hub's few strings in English, French and Malagasy.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    Fr,
    Mg,
}

impl Lang {
    pub fn from_tag(tag: &str) -> Self {
        match tag.split(['-', '_']).next().unwrap_or_default() {
            "fr" => Lang::Fr,
            "mg" => Lang::Mg,
            _ => Lang::En,
        }
    }
}

pub struct Text {
    pub subtitle: &'static str,
    pub open: &'static str,
    pub get: &'static str,
    pub soon: &'static str,
    pub installed: &'static str,
    pub source: &'static str,
}

pub fn text(lang: Lang) -> Text {
    match lang {
        Lang::Fr => Text {
            subtitle: "Des apps libres pour créer, hors ligne, sur votre appareil",
            open: "Ouvrir",
            get: "Télécharger",
            soon: "Bientôt",
            installed: "Installées",
            source: "Code source et versions",
        },
        Lang::Mg => Text {
            subtitle: "Rindranasa malalaka hamoronana, tsy mila aterineto, ao amin'ny fitaovanao",
            open: "Sokafana",
            get: "Alaina",
            soon: "Ho avy",
            installed: "Efa napetraka",
            source: "Kaody loharano sy dikan-teny",
        },
        Lang::En => Text {
            subtitle: "Free apps for making things, offline, on your device",
            open: "Open",
            get: "Download",
            soon: "Coming soon",
            installed: "Installed",
            source: "Source code and releases",
        },
    }
}

/// An app's tagline in the hub's language (the catalog's English otherwise).
pub fn tagline(lang: Lang, id: &str, english: &'static str) -> &'static str {
    let fr = match id {
        "sary" => "Retouche d'images",
        "taratasy" => "PDF : lire, annoter, signer",
        "soratra" => "Traitement de texte",
        "tabilao" => "Tableur",
        "seho" => "Présentations",
        "tsipika" => "Illustration vectorielle",
        "endrika" => "Design d'interfaces",
        "pejy" => "Mise en page",
        "hazavana" => "Photothèque et développement RAW",
        "feo" => "Studio audio",
        "hetsika" => "Montage vidéo",
        "tselatra" => "Animation et effets",
        "drafitra" => "Dessin technique (CAO)",
        _ => english,
    };
    let mg = match id {
        "sary" => "Fanamboarana sary",
        "taratasy" => "PDF: vakiana, asiana fanamarihana, soniavina",
        "soratra" => "Fanoratana lahatsoratra",
        "tabilao" => "Tabilao kajy",
        "seho" => "Fampisehoana",
        "tsipika" => "Sary vektoraly",
        "endrika" => "Famolavolana endrika",
        "pejy" => "Fandaminana pejy",
        "hazavana" => "Tahirin-tsary sy RAW",
        "feo" => "Studio feo",
        "hetsika" => "Fanamboarana horonantsary",
        "tselatra" => "Sary mihetsika sy effets",
        "drafitra" => "Sary teknika (CAO)",
        _ => english,
    };
    match lang {
        Lang::Fr => fr,
        Lang::Mg => mg,
        Lang::En => english,
    }
}
