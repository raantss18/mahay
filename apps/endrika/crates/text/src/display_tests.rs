use super::*;

/// Every shown char 10 units wide, left to right.
fn edges(v: &VisualLine) -> Vec<(f64, f64)> {
    let chars: Vec<(f64, f64)> = (0..v.text.chars().count()).map(|k| (k as f64 * 10.0, k as f64 * 10.0 + 10.0)).collect();
    v.cluster_edges(&chars)
}

#[test]
fn left_to_right_text_shows_as_is() {
    let v = visual_line("Frame 12");
    assert_eq!(v.text, "Frame 12");
    assert!(!v.rtl_para);
    assert_eq!(v.clusters.len(), 8);
    assert!(!has_rtl("Frame 12"));
    let e = edges(&v);
    assert_eq!(v.caret_x(0, &e), 0.0);
    assert_eq!(v.caret_x(8, &e), 80.0);
    assert_eq!(v.hit(14.0, &e), 1);
    assert_eq!(v.hit(16.0, &e), 2);
    assert_eq!(v.step(3, true, &e), 4);
    assert_eq!(v.step(0, false, &e), 0);
}

#[test]
fn hebrew_runs_right_to_left() {
    let v = visual_line("שלום");
    assert_eq!(v.text, "םולש");
    assert!(v.rtl_para);
    assert!(v.clusters.iter().all(|c| c.rtl));
    // The caret before the first letter is at the right end; the end of the text at the left.
    let e = edges(&v);
    assert_eq!(v.caret_x(0, &e), 40.0);
    assert_eq!(v.caret_x(4, &e), 0.0);
    // Right of the text is its start, left of it its end.
    assert_eq!(v.hit(100.0, &e), 0);
    assert_eq!(v.hit(-5.0, &e), 4);
    // The right arrow goes back towards the start.
    assert_eq!(v.step(2, true, &e), 1);
    assert_eq!(v.step(2, false, &e), 3);
}

#[test]
fn mixed_direction_names_order_and_caret() {
    // A left-to-right name with a Hebrew word.
    let v = visual_line("abc אבג");
    assert_eq!(v.text, "abc גבא");
    assert!(!v.rtl_para);
    let e = edges(&v);
    assert_eq!(v.caret_x(4, &e), 70.0);
    assert_eq!(v.caret_x(7, &e), 40.0);
    // Visual right-arrow order across the runs.
    let mut at = 0;
    let mut seen = vec![at];
    for _ in 0..7 {
        at = v.step(at, true, &e);
        seen.push(at);
    }
    assert_eq!(seen, vec![0, 1, 2, 3, 7, 6, 5, 4]);
    assert_eq!(v.hit(65.0, &e), 4);
    assert_eq!(v.hit(62.0, &e), 5);
    // A selection inside the Hebrew word is one span on screen.
    assert_eq!(v.selection(4, 6, &e), vec![(50.0, 70.0)]);
    // One across the runs is two.
    assert_eq!(v.selection(2, 5, &e), vec![(20.0, 40.0), (60.0, 70.0)]);

    // A right-to-left name with a Latin word: the Latin word keeps its order, at the left.
    let v = visual_line("אב abc");
    assert_eq!(v.text, "abc בא");
    assert!(v.rtl_para);
    let e = edges(&v);
    assert_eq!(v.caret_x(0, &e), 60.0);
    assert_eq!(v.caret_x(3, &e), 0.0);
    assert_eq!(v.caret_x(6, &e), 30.0);
}

#[test]
fn brackets_mirror_in_right_to_left_runs() {
    assert_eq!(visual_line("(שלום)").text, "(םולש)");
    assert_eq!(visual_line("(abc)").text, "(abc)");
}

#[test]
fn arabic_letters_join() {
    // Salam: seen (initial), lam-alef (final ligature), meem (isolated after the alef).
    let v = visual_line("سلام");
    assert_eq!(v.text, "\u{FEE1}\u{FEFC}\u{FEB3}");
    assert_eq!(v.clusters.len(), 3);
    assert_eq!(v.clusters.get(1).map(|c| c.logical.clone()), Some(1..3));
    // The caret between lam and alef sits in the middle of the ligature.
    let e = edges(&v);
    assert_eq!(v.caret_x(2, &e), 15.0);
    // Beh: isolated, initial, medial, final.
    assert_eq!(visual_line("ب").text, "\u{FE8F}");
    assert_eq!(visual_line("ببب").text, "\u{FE90}\u{FE92}\u{FE91}");
    // Marks stay with their letter and don't break joins.
    let v = visual_line("بَب");
    assert_eq!(v.text, "\u{FE90}\u{FE91}\u{064E}");
    assert_eq!(v.clusters.len(), 2);
    // Persian letters too.
    assert_eq!(visual_line("پی").text, "\u{FBFD}\u{FB58}");
}

#[test]
fn bidi_controls_show_nothing() {
    let v = visual_line("\u{200F}abc");
    assert!(!v.text.contains('\u{200F}'));
    assert_eq!(v.len, 4);
    let e = edges(&v);
    // Every logical caret position still has a place.
    for i in 0..=4 {
        assert!(v.caret_x(i, &e).is_finite());
    }
}

#[test]
fn elision_keeps_the_start_and_follows_the_direction() {
    let fits = |v: &VisualLine| v.text.chars().count() <= 5;
    assert_eq!(elide("Frame", fits).text, "Frame");
    assert_eq!(elide("Rectangle 12", fits).text, "Rect…");
    let v = elide("שלום עולם", fits);
    // The ellipsis is at the left of a right-to-left name; its right end (the start) stays.
    assert_eq!(v.text, "…םולש");
    // Nothing fits: just the ellipsis.
    assert_eq!(elide("abcdef", |v: &VisualLine| v.text.chars().count() <= 1).text, "…");
    // A letter is never cut from its marks.
    let v = elide("بَبَبَبَ", |v: &VisualLine| v.text.chars().count() <= 3);
    assert!(v.text.contains('\u{064E}'));
}

#[test]
fn junk_input_never_panics() {
    for s in ["", " ", "\u{0644}", "\u{064E}", "\u{0644}\u{0627}", "a\nב\tc", "\u{202E}\u{2067}x\u{2069}", "ـبـ", "‏‎‏"] {
        let v = visual_line(s);
        let e = edges(&v);
        for i in 0..=v.len + 2 {
            let _ = v.caret_x(i, &e);
            let _ = v.step(i, true, &e);
            let _ = v.step(i, false, &e);
        }
        let _ = v.hit(-1.0, &e);
        let _ = v.hit(1e9, &e);
        let _ = v.selection(0, v.len + 3, &e);
        let _ = elide(s, |t: &VisualLine| t.text.len() < 3);
        // Too few measured edges.
        let _ = v.caret_x(1, &[]);
        let _ = v.cluster_edges(&[]);
    }
}

/// The joined forms are the ones the shaper picks in context (when Noto Sans Arabic is built in).
#[test]
fn presentation_forms_match_the_shaper() {
    let Some(face) = crate::find_face("Noto Sans Arabic", "Regular").filter(|f| f.family == "Noto Sans Arabic") else {
        eprintln!("no Noto Sans Arabic; skipping");
        return;
    };
    let mut wrong = Vec::new();
    for (base, iso, n) in FORMS {
        // Isolated, then (dual-joining) initial, medial and final via tatweels around the letter.
        let mut cases = vec![(base.to_string(), iso as u32)];
        if n >= 2 {
            cases.push((format!("\u{0640}{base}"), iso as u32 + 1));
        }
        if n == 4 {
            cases.push((format!("{base}\u{0640}"), iso as u32 + 2));
            cases.push((format!("\u{0640}{base}\u{0640}"), iso as u32 + 3));
        }
        for (s, form) in cases {
            let form = char::from_u32(form).unwrap();
            assert!(face.has_char(form), "{base:?} form {form:?} missing");
            let mut t = figcraft_doc::TextData::new(s.clone());
            t.style.font_family = "Noto Sans Arabic".into();
            let l = crate::layout(&t, None, None);
            let at = s.chars().position(|c| c == base).unwrap();
            // The font may draw a letter as a skeleton plus dots: compare the cluster's advance.
            let shaped: f64 = l.glyphs.iter().filter(|g| g.char_start <= at && at < g.char_end).map(|g| g.advance).sum();
            // (Kerning and contextual alternates shift it a little: the right form is the nearest.)
            let adv = |c: u32| char::from_u32(c).map_or(f64::NAN, |c| face.advance(face.glyph_for(c)) * t.style.font_size / face.upem);
            let nearest =
                (iso as u32..iso as u32 + u32::from(n)).min_by(|a, b| (adv(*a) - shaped).abs().total_cmp(&(adv(*b) - shaped).abs())).unwrap();
            // Noto draws a lone heh wider than its isolated presentation form (a design choice).
            if (adv(nearest) - adv(form as u32)).abs() > 1e-9 && s != "\u{0647}" {
                wrong.push(format!("{base:?} in {s:?} should show as {form:?} ({shaped} vs {})", adv(form as u32)));
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
