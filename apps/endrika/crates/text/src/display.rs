//! One line of display text for UI toolkits that neither shape nor reorder text (layer, page, style
//! and file names in the panels): Unicode bidi reordering into visual order, mirrored brackets in
//! right-to-left runs, and Arabic letters in their contextual (joined) presentation forms, lam-alef
//! ligatures included. The result keeps, for every cluster on screen, the range of logical chars it
//! shows, so a caller that measured the shown text can place a caret, hit-test a click, move the
//! caret visually and draw a selection (see [`VisualLine::caret_x`] and friends).

use std::ops::Range;

use crate::layout::{is_rtl_char, visual_order};

/// What one shown cluster is: a base char with its combining marks (or a lam-alef pair).
#[derive(Clone, Debug, PartialEq)]
pub struct Cluster {
    /// Logical chars it shows.
    pub logical: Range<usize>,
    /// Chars of [`VisualLine::text`] it is drawn with (empty for invisible bidi controls).
    pub shown: Range<usize>,
    /// It runs right to left (odd bidi level).
    pub rtl: bool,
}

/// A line of text in display order.
#[derive(Clone, Debug, PartialEq)]
pub struct VisualLine {
    /// The text as shown, left to right.
    pub text: String,
    /// The shown clusters, left to right.
    pub clusters: Vec<Cluster>,
    /// The paragraph runs right to left (its first strong character does).
    pub rtl_para: bool,
    /// Chars in the logical text.
    pub len: usize,
}

/// Whether `s` has anything that reads right to left (else it shows as is).
pub fn has_rtl(s: &str) -> bool {
    s.chars().any(is_rtl_char)
}

/// `s` (one line; line breaks show as spaces) in display order.
pub fn visual_line(s: &str) -> VisualLine {
    let chars: Vec<char> = s.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    let len = chars.len();
    if !chars.iter().copied().any(is_rtl_char) {
        let clusters = (0..len).map(|i| Cluster { logical: i..i + 1, shown: i..i + 1, rtl: false }).collect();
        return VisualLine { text: chars.into_iter().collect(), clusters, rtl_para: false, len };
    }
    let flat: String = chars.iter().collect();
    let info = unicode_bidi::BidiInfo::new(&flat, None);
    let (levels, rtl_para) = match info.paragraphs.first() {
        Some(p) => {
            let lv = info.reordered_levels_per_char(p, p.range.clone());
            (lv.iter().map(|l| l.number()).collect::<Vec<u8>>(), p.level.is_rtl())
        }
        None => (vec![0; len], false),
    };
    let shaped = shape_arabic(&chars);
    // Clusters in logical order: a base with the marks after it; lam-alef as one.
    let mut logical: Vec<(Range<usize>, String, u8)> = Vec::new();
    let mut i = 0;
    while i < len {
        let start = i;
        let mut shown = String::new();
        match shaped.get(i).copied().flatten() {
            Some(Shaped::Ligature(c)) => {
                shown.push(c);
                i += 2;
            }
            Some(Shaped::Form(c)) => {
                shown.push(c);
                i += 1;
            }
            None => {
                if let Some(c) = chars.get(i).copied().filter(|c| !is_bidi_control(*c)) {
                    shown.push(c);
                }
                i += 1;
            }
        }
        while let Some(c) = chars.get(i).copied().filter(|c| joining(*c) == Join::Transparent) {
            shown.push(c);
            i += 1;
        }
        let level = levels.get(start).copied().unwrap_or(0);
        if level % 2 == 1 {
            shown = shown.chars().map(mirror).collect();
        }
        logical.push((start..i.min(len), shown, level));
    }
    let order = visual_order(&logical.iter().map(|c| c.2).collect::<Vec<u8>>());
    let mut text = String::new();
    let mut clusters = Vec::with_capacity(order.len());
    let mut at = 0;
    for k in order {
        let Some((range, shown, level)) = logical.get(k) else { continue };
        let n = shown.chars().count();
        text.push_str(shown);
        clusters.push(Cluster { logical: range.clone(), shown: at..at + n, rtl: level % 2 == 1 });
        at += n;
    }
    VisualLine { text, clusters, rtl_para, len }
}

/// The longest start of `s` that, with an ellipsis after it, `fits` (given how it shows); `s`
/// itself when it fits. The ellipsis follows the paragraph: at the left end of a right-to-left name.
pub fn elide(s: &str, fits: impl Fn(&VisualLine) -> bool) -> VisualLine {
    let full = visual_line(s);
    if fits(&full) {
        return full;
    }
    // Cut only between clusters (never between a letter and its marks).
    let chars: Vec<char> = s.chars().collect();
    let cuts: Vec<usize> = (1..chars.len()).filter(|i| chars.get(*i).is_some_and(|c| joining(*c) != Join::Transparent)).collect();
    let with = |n: usize| -> VisualLine {
        let mut t: String = chars.iter().take(n).collect::<String>().trim_end().to_string();
        t.push('…');
        visual_line(&t)
    };
    // Binary search the number of cuts kept.
    let (mut lo, mut hi) = (0usize, cuts.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if cuts.get(mid - 1).is_some_and(|n| fits(&with(*n))) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    match lo.checked_sub(1).and_then(|k| cuts.get(k)) {
        Some(n) => with(*n),
        None => visual_line("…"),
    }
}

impl VisualLine {
    /// Each cluster's left and right x from the measured left and right x of every shown char.
    /// An invisible cluster sits where the one before it ends.
    pub fn cluster_edges(&self, char_edges: &[(f64, f64)]) -> Vec<(f64, f64)> {
        let mut x = char_edges.first().map_or(0.0, |e| e.0);
        self.clusters
            .iter()
            .map(|c| {
                let l = char_edges.get(c.shown.start).map_or(x, |e| e.0);
                let r = c.shown.end.checked_sub(1).filter(|_| !c.shown.is_empty()).and_then(|k| char_edges.get(k)).map_or(l, |e| e.1);
                x = r;
                (l, r)
            })
            .collect()
    }

    /// The x of a caret before logical char `i` (`len` = the end), given [`cluster_edges`](Self::cluster_edges).
    /// It sits at the leading edge of the char after it (the right edge in a right-to-left run); at
    /// the end, after the last char.
    pub fn caret_x(&self, i: usize, edges: &[(f64, f64)]) -> f64 {
        let i = i.min(self.len);
        let at = |k: usize, frac: f64| -> Option<f64> {
            let c = self.clusters.get(k)?;
            let (l, r) = edges.get(k).copied()?;
            Some(if c.rtl { r - (r - l) * frac } else { l + (r - l) * frac })
        };
        if let Some((k, c)) = self.clusters.iter().enumerate().find(|(_, c)| c.logical.contains(&i)) {
            let frac = (i - c.logical.start) as f64 / c.logical.len().max(1) as f64;
            return at(k, frac).unwrap_or(0.0);
        }
        // The end: after the last logical cluster.
        match self.clusters.iter().position(|c| c.logical.end == self.len) {
            Some(k) => at(k, 1.0).unwrap_or(0.0),
            None => edges.first().map_or(0.0, |e| e.0),
        }
    }

    /// The logical caret position nearest to `x`.
    pub fn hit(&self, x: f64, edges: &[(f64, f64)]) -> usize {
        for (c, (l, r)) in self.clusters.iter().zip(edges) {
            if x < (l + r) / 2.0 {
                return if c.rtl { c.logical.end } else { c.logical.start };
            }
            if x < *r {
                return if c.rtl { c.logical.start } else { c.logical.end };
            }
        }
        // Past the right end: the end of a left-to-right line, the start of a right-to-left one.
        match self.clusters.last() {
            Some(c) if c.rtl => c.logical.start,
            Some(c) => c.logical.end,
            None => 0,
        }
    }

    /// The caret one step left or right of `i` on screen (the nearest caret position past it).
    /// At the line's edge the caret stays.
    pub fn step(&self, i: usize, right: bool, edges: &[(f64, f64)]) -> usize {
        let x = self.caret_x(i, edges);
        (0..=self.len)
            .filter(|k| *k != i)
            .map(|k| (k, self.caret_x(k, edges)))
            .filter(|(_, cx)| if right { *cx > x + 0.01 } else { *cx < x - 0.01 })
            .min_by(|a, b| (a.1 - x).abs().total_cmp(&(b.1 - x).abs()).then(a.0.abs_diff(i).cmp(&b.0.abs_diff(i))))
            .map_or(i, |(k, _)| k)
    }

    /// The x spans (left, right) covering logical chars `a..b`, left to right; a selection across
    /// direction runs can be several spans.
    pub fn selection(&self, a: usize, b: usize, edges: &[(f64, f64)]) -> Vec<(f64, f64)> {
        let (a, b) = (a.min(b), a.max(b));
        let mut out: Vec<(f64, f64)> = Vec::new();
        for (c, (l, r)) in self.clusters.iter().zip(edges) {
            let (s, e) = (c.logical.start.max(a), c.logical.end.min(b));
            if s >= e {
                continue;
            }
            // Part of a cluster (a lam-alef ligature): its share, from the leading edge.
            let n = c.logical.len().max(1) as f64;
            let (f0, f1) = ((s - c.logical.start) as f64 / n, (e - c.logical.start) as f64 / n);
            let (x0, x1) = if c.rtl { (r - (r - l) * f1, r - (r - l) * f0) } else { (l + (r - l) * f0, l + (r - l) * f1) };
            match out.last_mut() {
                Some(last) if (last.1 - x0).abs() < 0.5 => last.1 = x1,
                _ => out.push((x0, x1)),
            }
        }
        out
    }
}

/// Explicit bidi formatting characters: they steer the order but show nothing.
fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{200E}' | '\u{200F}' | '\u{061C}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
}

/// The mirrored glyph of a paired character shown in a right-to-left run.
fn mirror(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '«' => '»',
        '»' => '«',
        '‹' => '›',
        '›' => '‹',
        '≤' => '≥',
        '≥' => '≤',
        other => other,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Join {
    /// Doesn't join (spaces, Latin, hamza…).
    None,
    /// Joins only to the char before it (alef, dal, reh, waw…).
    Right,
    /// Joins on both sides (beh, seen, lam…).
    Dual,
    /// Joins whatever is next to it but has no forms (tatweel, zero width joiner).
    Causing,
    /// Combining marks: skipped when deciding joins.
    Transparent,
}

/// Arabic letters and their presentation forms (isolated, final, initial, medial; the first two
/// only for right-joining letters).
const FORMS: [(char, char, u8); 51] = [
    ('\u{0621}', '\u{FE80}', 1),
    ('\u{0622}', '\u{FE81}', 2),
    ('\u{0623}', '\u{FE83}', 2),
    ('\u{0624}', '\u{FE85}', 2),
    ('\u{0625}', '\u{FE87}', 2),
    ('\u{0626}', '\u{FE89}', 4),
    ('\u{0627}', '\u{FE8D}', 2),
    ('\u{0628}', '\u{FE8F}', 4),
    ('\u{0629}', '\u{FE93}', 2),
    ('\u{062A}', '\u{FE95}', 4),
    ('\u{062B}', '\u{FE99}', 4),
    ('\u{062C}', '\u{FE9D}', 4),
    ('\u{062D}', '\u{FEA1}', 4),
    ('\u{062E}', '\u{FEA5}', 4),
    ('\u{062F}', '\u{FEA9}', 2),
    ('\u{0630}', '\u{FEAB}', 2),
    ('\u{0631}', '\u{FEAD}', 2),
    ('\u{0632}', '\u{FEAF}', 2),
    ('\u{0633}', '\u{FEB1}', 4),
    ('\u{0634}', '\u{FEB5}', 4),
    ('\u{0635}', '\u{FEB9}', 4),
    ('\u{0636}', '\u{FEBD}', 4),
    ('\u{0637}', '\u{FEC1}', 4),
    ('\u{0638}', '\u{FEC5}', 4),
    ('\u{0639}', '\u{FEC9}', 4),
    ('\u{063A}', '\u{FECD}', 4),
    ('\u{0641}', '\u{FED1}', 4),
    ('\u{0642}', '\u{FED5}', 4),
    ('\u{0643}', '\u{FED9}', 4),
    ('\u{0644}', '\u{FEDD}', 4),
    ('\u{0645}', '\u{FEE1}', 4),
    ('\u{0646}', '\u{FEE5}', 4),
    ('\u{0647}', '\u{FEE9}', 4),
    ('\u{0648}', '\u{FEED}', 2),
    ('\u{0649}', '\u{FEEF}', 2),
    ('\u{064A}', '\u{FEF1}', 4),
    ('\u{0671}', '\u{FB50}', 2),
    ('\u{0679}', '\u{FB66}', 4),
    ('\u{067E}', '\u{FB56}', 4),
    ('\u{0686}', '\u{FB7A}', 4),
    ('\u{0688}', '\u{FB88}', 2),
    ('\u{0691}', '\u{FB8C}', 2),
    ('\u{0698}', '\u{FB8A}', 2),
    ('\u{06A9}', '\u{FB8E}', 4),
    ('\u{06AF}', '\u{FB92}', 4),
    ('\u{06BA}', '\u{FB9E}', 2),
    ('\u{06BE}', '\u{FBAA}', 4),
    ('\u{06C1}', '\u{FBA6}', 4),
    ('\u{06CC}', '\u{FBFC}', 4),
    ('\u{06D2}', '\u{FBAE}', 2),
    ('\u{06D3}', '\u{FBB0}', 2),
];

/// Lam-alef ligatures: (alef, isolated form; the final form follows it).
const LAM_ALEF: [(char, char); 4] = [('\u{0622}', '\u{FEF5}'), ('\u{0623}', '\u{FEF7}'), ('\u{0625}', '\u{FEF9}'), ('\u{0627}', '\u{FEFB}')];

fn forms(c: char) -> Option<(char, u8)> {
    FORMS.iter().find(|f| f.0 == c).map(|f| (f.1, f.2))
}

fn joining(c: char) -> Join {
    match c {
        '\u{0610}'..='\u{061A}'
        | '\u{064B}'..='\u{065F}'
        | '\u{0670}'
        | '\u{06D6}'..='\u{06DC}'
        | '\u{06DF}'..='\u{06E4}'
        | '\u{06E7}'
        | '\u{06E8}'
        | '\u{06EA}'..='\u{06ED}' => Join::Transparent,
        '\u{0640}' | '\u{200D}' => Join::Causing,
        _ => match forms(c) {
            Some((_, 4)) => Join::Dual,
            Some((_, 2)) => Join::Right,
            _ => Join::None,
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shaped {
    /// The char's contextual form.
    Form(char),
    /// A lam-alef ligature standing for this char and the next.
    Ligature(char),
}

/// Contextual forms of the Arabic letters in `chars` (None = shown as is; the alef of a lam-alef
/// ligature is None too, the ligature stands at the lam).
fn shape_arabic(chars: &[char]) -> Vec<Option<Shaped>> {
    let mut out = vec![None; chars.len()];
    let kinds: Vec<Join> = chars.iter().map(|c| joining(*c)).collect();
    // The nearest non-transparent neighbour's joining type.
    let prev = |i: usize| (0..i).rev().map(|k| kinds.get(k).copied().unwrap_or(Join::None)).find(|j| *j != Join::Transparent).unwrap_or(Join::None);
    let next =
        |i: usize| (i + 1..chars.len()).map(|k| kinds.get(k).copied().unwrap_or(Join::None)).find(|j| *j != Join::Transparent).unwrap_or(Join::None);
    let mut i = 0;
    while i < chars.len() {
        let (Some(c), Some(kind)) = (chars.get(i).copied(), kinds.get(i).copied()) else { break };
        let joins_before = matches!(prev(i), Join::Dual | Join::Causing);
        if c == '\u{0644}'
            && let Some(lig) = chars.get(i + 1).and_then(|a| LAM_ALEF.iter().find(|l| l.0 == *a)).map(|l| l.1)
        {
            let form = if joins_before { char::from_u32(lig as u32 + 1).unwrap_or(lig) } else { lig };
            if let Some(slot) = out.get_mut(i) {
                *slot = Some(Shaped::Ligature(form));
            }
            i += 2;
            continue;
        }
        if let Some((iso, n)) = forms(c) {
            let joins_after = kind == Join::Dual && matches!(next(i), Join::Right | Join::Dual | Join::Causing);
            let offset = match (n, joins_before, joins_after) {
                (1, _, _) => 0,
                (_, false, false) => 0,
                (_, true, false) => 1,
                (_, false, true) => 2,
                (_, true, true) => 3,
            };
            if let (Some(slot), Some(f)) = (out.get_mut(i), char::from_u32(iso as u32 + offset)) {
                *slot = Some(Shaped::Form(f));
            }
        }
        i += 1;
    }
    out
}

#[cfg(test)]
#[path = "display_tests.rs"]
mod tests;
