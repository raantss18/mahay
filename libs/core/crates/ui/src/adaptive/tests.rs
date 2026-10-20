use egui::{Context, RawInput, Rect, Sense, pos2, vec2};

use super::*;

fn frame(ctx: &Context, width: f32, ui_fn: impl FnMut(&mut egui::Ui)) {
    let input = RawInput { screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(width, 600.0))), ..Default::default() };
    let mut ui_fn = ui_fn;
    for _ in 0..2 {
        let mut out = ctx.run_ui(input.clone(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| ui_fn(ui));
        });
        out.textures_delta.clear();
    }
}

#[test]
fn thresholds_are_600_and_840() {
    assert_eq!(FormFactor::from_width(360.0), FormFactor::Compact);
    assert_eq!(FormFactor::from_width(411.0), FormFactor::Compact);
    assert_eq!(FormFactor::from_width(599.9), FormFactor::Compact);
    assert_eq!(FormFactor::from_width(600.0), FormFactor::Medium);
    assert_eq!(FormFactor::from_width(839.9), FormFactor::Medium);
    assert_eq!(FormFactor::from_width(840.0), FormFactor::Expanded);
}

#[test]
fn hysteresis_stops_flicker_at_a_threshold() {
    let mut class = FormFactor::Expanded;
    let mut seen = Vec::new();
    for w in [900.0, 845.0, 839.0, 835.0, 833.0, 831.0, 835.0, 845.0, 849.0, 841.0] {
        class = FormFactor::next(class, w);
        seen.push(class);
    }
    // Expanded until 831 (< 832), back only at 848 or more.
    use FormFactor::*;
    assert_eq!(seen, [Expanded, Expanded, Expanded, Expanded, Expanded, Medium, Medium, Medium, Expanded, Expanded]);
    // A width resting on 600 for a hundred frames never changes the class.
    let mut c = FormFactor::Compact;
    for _ in 0..100 {
        c = FormFactor::next(c, 600.0);
    }
    assert_eq!(c, FormFactor::Compact);
    // It is stable when applied to its own result.
    for w in [0.0, 300.0, 592.0, 600.0, 608.0, 700.0, 832.0, 840.0, 848.0, 2000.0] {
        for start in [FormFactor::Compact, FormFactor::Medium, FormFactor::Expanded] {
            let once = FormFactor::next(start, w);
            assert_eq!(FormFactor::next(once, w), once, "{start:?} at {w}");
        }
    }
}

#[test]
fn of_follows_the_window_every_frame() {
    let ctx = Context::default();
    let mut seen = Vec::new();
    for w in [360.0, 700.0, 1000.0, 700.0, 360.0] {
        frame(&ctx, w, |ui| seen.push(FormFactor::of_ui(ui)));
    }
    seen.dedup();
    assert_eq!(seen, [FormFactor::Compact, FormFactor::Medium, FormFactor::Expanded, FormFactor::Medium, FormFactor::Compact]);
}

#[test]
fn audit_sees_overlap_and_off_screen_widgets() {
    let ctx = Context::default();
    frame(&ctx, 360.0, |ui| {
        let at = ui.max_rect().min;
        ui.put(Rect::from_min_size(at, vec2(100.0, 40.0)), egui::Button::new("a"));
        ui.put(Rect::from_min_size(at + vec2(60.0, 0.0), vec2(100.0, 40.0)), egui::Button::new("b"));
        ui.put(Rect::from_min_size(pos2(300.0, 100.0), vec2(100.0, 40.0)), egui::Button::new("c"));
    });
    let kinds: Vec<_> = audit(&ctx).iter().map(|f| f.kind).collect();
    assert!(kinds.contains(&FindingKind::Overlap), "{kinds:?}");
    assert!(kinds.contains(&FindingKind::OutsideWindow), "{kinds:?}");
}

#[test]
fn audit_passes_a_scrolling_bar_wider_than_the_window() {
    let ctx = Context::default();
    frame(&ctx, 360.0, |ui| {
        command_bar(ui, "bar", |ui| {
            for i in 0..30 {
                ui.add(egui::Button::new(format!("Command {i}")).min_size(vec2(0.0, MIN_TOUCH)));
            }
        });
    });
    assert!(audit(&ctx).is_empty(), "{:?}", audit(&ctx));
    assert!(small_targets(&ctx, MIN_TOUCH).is_empty(), "{:?}", small_targets(&ctx, MIN_TOUCH));
}

#[test]
fn a_nested_background_is_not_an_overlap() {
    let ctx = Context::default();
    frame(&ctx, 360.0, |ui| {
        let r = Rect::from_min_size(ui.max_rect().min, vec2(300.0, 60.0));
        ui.interact(r, egui::Id::new("bg"), Sense::click());
        ui.put(Rect::from_min_size(r.min + vec2(10.0, 10.0), vec2(80.0, 40.0)), egui::Button::new("x"));
    });
    assert!(audit(&ctx).is_empty());
}

#[test]
fn a_thin_resize_grip_over_an_edge_is_not_an_overlap() {
    let ctx = Context::default();
    frame(&ctx, 360.0, |ui| {
        let at = ui.max_rect().min;
        ui.put(Rect::from_min_size(at, vec2(200.0, 60.0)), egui::Button::new("list"));
        ui.interact(Rect::from_min_size(at + vec2(197.0, 0.0), vec2(6.0, 60.0)), egui::Id::new("grip"), Sense::click());
    });
    assert!(audit(&ctx).is_empty(), "{:?}", audit(&ctx));
}

#[test]
fn a_quick_move_pans_and_a_long_rest_selects() {
    let g = |mode| Gesture { start: pos2(100.0, 100.0), started_at: 10.0, mode };
    assert_eq!(decide(&g(Mode::Undecided), pos2(103.0, 104.0), 10.1), Mode::Undecided);
    assert_eq!(decide(&g(Mode::Undecided), pos2(100.0, 130.0), 10.1), Mode::Panning);
    assert_eq!(decide(&g(Mode::Undecided), pos2(100.0, 130.0), 10.6), Mode::Selecting);
    assert_eq!(decide(&g(Mode::Undecided), pos2(101.0, 100.0), 10.6), Mode::Selecting);
    assert_eq!(decide(&g(Mode::Panning), pos2(100.0, 100.0), 12.0), Mode::Panning);
    assert_eq!(decide(&g(Mode::Ignored), pos2(300.0, 100.0), 10.1), Mode::Ignored);
}

/// A finger drag over the area pans it, with the coast after a lift.
#[test]
fn a_finger_pan_follows_the_finger_and_coasts() {
    use egui::{Event, PointerButton, TouchDeviceId, TouchId, TouchPhase};
    let ctx = Context::default();
    let mut pan = FingerPan::default();
    let mut total = egui::Vec2::ZERO;
    let mut coasted = egui::Vec2::ZERO;
    let area = Rect::from_min_size(pos2(0.0, 0.0), vec2(360.0, 600.0));
    let mut time = 0.0;
    let mut run = |events: Vec<Event>, pan: &mut FingerPan| -> PanStep {
        time += 1.0 / 60.0;
        let input = RawInput { screen_rect: Some(area), time: Some(time), events, ..Default::default() };
        let mut step = PanStep::default();
        let mut out = ctx.run_ui(input, |ui| step = pan.update(&ui.input(|i| i.clone()), area, true));
        out.textures_delta.clear();
        step
    };
    let touch = |phase, pos| Event::Touch { device_id: TouchDeviceId(0), id: TouchId(0), phase, pos, force: None };
    let start = pos2(200.0, 400.0);
    run(vec![touch(TouchPhase::Start, start), Event::PointerMoved(start), Event::PointerButton { pos: start, button: PointerButton::Primary, pressed: true, modifiers: Default::default() }], &mut pan);
    let mut at = start;
    for _ in 0..8 {
        at.y -= 20.0;
        let step = run(vec![touch(TouchPhase::Move, at), Event::PointerMoved(at)], &mut pan);
        total += step.delta;
    }
    assert!(total.y < -100.0, "the content followed the finger up: {total:?}");
    let lift = run(vec![touch(TouchPhase::End, at), Event::PointerButton { pos: at, button: PointerButton::Primary, pressed: false, modifiers: Default::default() }], &mut pan);
    coasted += lift.delta;
    for _ in 0..30 {
        coasted += run(vec![], &mut pan).delta;
    }
    assert!(coasted.y < -20.0, "a fast flick coasts on: {coasted:?}");
}

#[test]
fn height_class_follows_the_window_with_hysteresis() {
    assert_eq!(HeightClass::from_height(412.0), HeightClass::Compact);
    assert_eq!(HeightClass::from_height(480.0), HeightClass::Medium);
    assert_eq!(HeightClass::from_height(900.0), HeightClass::Expanded);
    let mut c = HeightClass::Medium;
    for _ in 0..100 {
        c = HeightClass::next(c, 480.0);
    }
    assert_eq!(c, HeightClass::Medium, "resting on a threshold does not flicker");
    assert_eq!(HeightClass::next(HeightClass::Medium, 475.0), HeightClass::Medium);
    assert_eq!(HeightClass::next(HeightClass::Medium, 470.0 - 40.0), HeightClass::Compact);
}

#[test]
fn window_size_is_recomputed_every_frame() {
    let ctx = Context::default();
    let mut seen = Vec::new();
    for (w, h) in [(411.0, 890.0), (890.0, 411.0), (600.0, 1000.0), (360.0, 780.0)] {
        let input = RawInput { screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(w, h))), ..Default::default() };
        let mut out = ctx.run_ui(input, |ui| seen.push(WindowSize::of(ui.ctx())));
        out.textures_delta.clear();
    }
    assert_eq!((seen[0].width_class, seen[0].height_class), (FormFactor::Compact, HeightClass::Medium));
    assert_eq!((seen[1].width_class, seen[1].height_class), (FormFactor::Expanded, HeightClass::Compact));
    assert!(seen[1].is_short());
    assert_eq!((seen[2].width_class, seen[2].height_class), (FormFactor::Medium, HeightClass::Expanded));
}

#[test]
fn columns_and_panels_follow_the_room() {
    assert_eq!(columns(360.0, 160.0, 8.0, 4), 2);
    assert_eq!(columns(411.0, 260.0, 8.0, 4), 1);
    assert_eq!(columns(900.0, 200.0, 8.0, 4), 4);
    assert_eq!(columns(0.0, 200.0, 8.0, 4), 1);
    assert_eq!(columns(f32::NAN, 200.0, 8.0, 4), 1);
    assert_eq!(panels_beside(411.0, 300.0, &[250.0, 250.0]), 0);
    assert_eq!(panels_beside(700.0, 300.0, &[250.0, 250.0]), 1);
    assert_eq!(panels_beside(900.0, 300.0, &[250.0, 250.0]), 2);
}
