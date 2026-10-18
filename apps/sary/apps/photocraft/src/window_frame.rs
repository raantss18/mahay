//! Keeps the custom (undecorated) title bar window truly borderless (#2246).
//!
//! winit 0.30 gives an undecorated window the `WS_CAPTION | WS_BORDER` styles and hides the
//! non-client area by spreading the client rect over it (`WM_NCCALCSIZE`). On some systems
//! (reported: Windows 10 LTSB, Intel HD Graphics 4600, 1366 × 768) the desktop compositor still
//! reserves that frame for the window surface: the content is drawn offset by the caption and
//! border, leaving black bars at the window's top-left, while the pointer keeps the un-offset
//! client coordinates, so it no longer points at what is drawn under it.
//!
//! winit 0.27 — before rust-windowing/winit#2419 — simply omitted those styles for undecorated
//! windows, and there was no offset. [`service`] restores that: it strips `WS_CAPTION` and
//! `WS_BORDER` and forces Windows to recalculate the frame. winit puts them back whenever it
//! changes the window flags (maximize, restore, …), so the returned guard checks every frame.
//!
//! Windows sizes a maximize by the frame its styles describe, so a window maximized while its
//! caption is gone is sized without one: its rect covers the whole monitor, taskbar included.
//! The shell reads a caption-less window covering its monitor as a full-screen app and drops the
//! taskbar's backdrop behind it — a black band on stock Windows 11, a glitched unresponsive one
//! under ExplorerPatcher (#3144). The guard therefore also trims a parked maximize's rect back to
//! the work area plus its own frame, clear of the taskbar. winit pins the client area to the work
//! area either way (`WM_NCCALCSIZE`), so the app's pixels do not move.
//!
//! The Windows calls go through safe `winsafe` wrappers; the workspace forbids `unsafe`, and
//! winsafe can't wrap a raw handle without it, so the window is found by its handle instead.

pub use platform::service;

#[cfg(target_os = "windows")]
mod platform {
    use eframe::wgpu::rwh::{HasWindowHandle, RawWindowHandle};
    use photocraft_ui_egui::WindowFrameFn;
    use winsafe::{HWND, HwndPlace, POINT, SIZE, co};

    /// A guard that keeps the window borderless each frame, or `None` when the app uses the
    /// system title bar (the window is decorated) or its handle can't be found.
    pub fn service(cc: &eframe::CreationContext<'_>, custom_titlebar: bool) -> Option<WindowFrameFn> {
        if !custom_titlebar {
            return None;
        }
        let target = match cc.window_handle().ok()?.as_raw() {
            RawWindowHandle::Win32(h) => h.hwnd.get(),
            _ => return None,
        };
        let hwnd = find_window(target)?;
        reapply(&hwnd);
        Some(Box::new(move || reapply(&hwnd)))
    }

    /// The top-level window with the given raw handle. `EnumWindows` hands out real `HWND`s, so
    /// this matches winit's own window without wrapping a raw handle (which needs `unsafe`).
    fn find_window(target: isize) -> Option<HWND> {
        let mut found = None;
        let _ = winsafe::EnumWindows(|hwnd| {
            if hwnd.ptr() as isize == target {
                found = Some(hwnd);
                false
            } else {
                true
            }
        });
        found
    }

    /// One pass of the per-frame guard: the styles winit put back come off again, and a parked
    /// maximize stays clear of the taskbar.
    fn reapply(hwnd: &HWND) {
        strip(hwnd);
        keep_clear_of_the_taskbar(hwnd);
    }

    /// Removes `WS_CAPTION`/`WS_BORDER` and has Windows recalculate the frame. A no-op once they
    /// are gone, so it is cheap to call every frame.
    fn strip(hwnd: &HWND) {
        let style = hwnd.style();
        let without = borderless(style);
        if without.raw() == style.raw() {
            return;
        }
        hwnd.set_style(without);
        let _ = hwnd.SetWindowPos(
            HwndPlace::None,
            POINT::new(),
            SIZE::new(),
            co::SWP::FRAMECHANGED | co::SWP::NOCOPYBITS | co::SWP::NOMOVE | co::SWP::NOSIZE | co::SWP::NOZORDER | co::SWP::NOACTIVATE,
        );
    }

    /// How far a parked maximize reaches past a monitor/work-area edge: the resize
    /// border (about 8 pt at 100 %, more with DPI). Past that, a maximized window's rect is not
    /// parked but being dragged or snapped, and is left alone.
    const FRAME: i32 = 32;

    /// Keep a parked maximize inside all four work-area edges, including a top taskbar.
    fn keep_clear_of_the_taskbar(hwnd: &HWND) {
        if !hwnd.IsZoomed() {
            return;
        }
        let (Ok(r), Ok(mi)) = (hwnd.GetWindowRect(), hwnd.MonitorFromWindow(co::MONITOR::DEFAULTTONEAREST).GetMonitorInfo()) else {
            return;
        };
        let (m, w) = (mi.rcMonitor, mi.rcWork);
        let Some((left, top, right, bottom)) =
            clamped_rect((r.left, r.top, r.right, r.bottom), (m.left, m.top, m.right, m.bottom), (w.left, w.top, w.right, w.bottom))
        else {
            return;
        };
        let (Some(width), Some(height)) = (right.checked_sub(left), bottom.checked_sub(top)) else {
            return;
        };
        // Moving the outer frame while winit pins the client rect to the work area must
        // not copy the old frame's pixels into the custom title bar.
        let _ = hwnd.SetWindowPos(
            HwndPlace::None,
            POINT { x: left, y: top },
            SIZE { cx: width, cy: height },
            co::SWP::FRAMECHANGED | co::SWP::NOCOPYBITS | co::SWP::NOZORDER | co::SWP::NOACTIVATE,
        );
        let _ = hwnd.InvalidateRect(None, false);
    }

    /// Return the corrected outer rect in desktop coordinates, preserving the resize border.
    /// Each edge must be parked just outside either the monitor or work-area edge. Accepting
    /// both makes the guard idempotent and handles Windows already correcting some edges.
    /// Dragged/snapped windows are left alone; no monitor-origin or DPI assumptions are needed.
    fn clamped_rect(rect: (i32, i32, i32, i32), monitor: (i32, i32, i32, i32), work: (i32, i32, i32, i32)) -> Option<(i32, i32, i32, i32)> {
        let (l, t, r, b) = rect;
        let (ml, mt, mr, mb) = monitor;
        let (wl, wt, wr, wb) = work;
        if ml >= mr || mt >= mb || wl < ml || wt < mt || wr > mr || wb > mb || wl >= wr || wt >= wb {
            return None;
        }
        // Infer the frame from an edge without a taskbar. On an obstructed edge, the
        // monitor and work-area offsets can both look like valid borders (e.g. auto-hide).
        let border = [
            (ml == wl).then(|| ml.checked_sub(l)).flatten(),
            (mt == wt).then(|| mt.checked_sub(t)).flatten(),
            (mr == wr).then(|| r.checked_sub(mr)).flatten(),
            (mb == wb).then(|| b.checked_sub(mb)).flatten(),
        ]
        .into_iter()
        .flatten()
        .filter(|n| (0..=FRAME).contains(n))
        .min()?;
        let parked = |monitor: Option<i32>, work: Option<i32>| monitor == Some(border) || work == Some(border);
        if !parked(ml.checked_sub(l), wl.checked_sub(l))
            || !parked(mt.checked_sub(t), wt.checked_sub(t))
            || !parked(r.checked_sub(mr), r.checked_sub(wr))
            || !parked(b.checked_sub(mb), b.checked_sub(wb))
        {
            return None;
        }
        let corrected = (wl.checked_sub(border)?, wt.checked_sub(border)?, wr.checked_add(border)?, wb.checked_add(border)?);
        (corrected != rect).then_some(corrected)
    }

    /// `style` without the OS title bar and frame (`WS_CAPTION`, which already includes
    /// `WS_BORDER`). The sizing border (`WS_THICKFRAME`) stays, so edge resizing and Aero snap
    /// keep working, as they did with winit 0.27's undecorated windows.
    fn borderless(style: co::WS) -> co::WS {
        style & !(co::WS::CAPTION | co::WS::BORDER)
    }

    #[cfg(test)]
    mod tests {
        use super::{borderless, clamped_rect};
        use winsafe::co;

        #[test]
        fn the_borderless_style_drops_the_caption_and_border_only() {
            // `WS_CAPTION` is 0x00c0_0000 and already contains `WS_BORDER` (0x0080_0000).
            let decorated = co::WS::CAPTION | co::WS::BORDER | co::WS::SYSMENU | co::WS::THICKFRAME | co::WS::MAXIMIZEBOX | co::WS::MINIMIZEBOX;
            let bare = borderless(decorated);
            assert!(!bare.has(co::WS::CAPTION), "caption gone");
            assert!(!bare.has(co::WS::BORDER), "border gone");
            assert!(bare.has(co::WS::THICKFRAME), "edge resizing kept");
            assert!(bare.has(co::WS::MAXIMIZEBOX) && bare.has(co::WS::SYSMENU), "window buttons kept");
            // Already borderless: unchanged, so the per-frame call does nothing.
            assert_eq!(bare.raw(), borderless(bare).raw());
        }

        #[test]
        fn a_maximize_respects_taskbars_on_every_edge() {
            let monitor = (0, 0, 1920, 1080);
            let rect = (-8, -8, 1928, 1088);
            for (work, expected) in [
                ((0, 0, 1920, 1046), (-8, -8, 1928, 1054)),
                ((0, 48, 1920, 1080), (-8, 40, 1928, 1088)),
                ((0, 2, 1920, 1080), (-8, -6, 1928, 1088)),
                ((0, 0, 1920, 1078), (-8, -8, 1928, 1086)),
                ((48, 0, 1920, 1080), (40, -8, 1928, 1088)),
                ((0, 0, 1872, 1080), (-8, -8, 1880, 1088)),
            ] {
                assert_eq!(clamped_rect(rect, monitor, work), Some(expected));
                assert_eq!(clamped_rect(expected, monitor, work), None, "already corrected");
            }
        }

        #[test]
        fn a_top_taskbar_keeps_its_offset_at_other_origins_and_frame_sizes() {
            let monitor = (-1920, -1080, 0, 0);
            let work = (-1920, -1032, 0, 0);
            for border in [0, 8, 16, 32] {
                let rect = (-1920 - border, -1080 - border, border, border);
                let expected = (-1920 - border, -1032 - border, border, border);
                assert_eq!(clamped_rect(rect, monitor, work), Some(expected));
                assert_eq!(clamped_rect(expected, monitor, work), None);
            }
        }

        #[test]
        fn a_maximize_without_a_reserved_taskbar_is_left_alone() {
            let monitor = (0, 0, 1920, 1080);
            assert_eq!(clamped_rect((-8, -8, 1928, 1088), monitor, monitor), None);
        }

        #[test]
        fn a_maximize_being_dragged_or_snapped_is_left_alone() {
            let monitor = (0, 0, 1920, 1080);
            let work = (0, 48, 1920, 1080);
            assert_eq!(clamped_rect((100, 200, 1100, 900), monitor, work), None, "moved");
            assert_eq!(clamped_rect((-500, -8, 1428, 1088), monitor, work), None, "dragged");
            assert_eq!(clamped_rect((0, 48, 960, 1080), monitor, work), None, "half-screen");
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use photocraft_ui_egui::WindowFrameFn;

    pub fn service(_cc: &eframe::CreationContext<'_>, _custom_titlebar: bool) -> Option<WindowFrameFn> {
        None
    }
}
