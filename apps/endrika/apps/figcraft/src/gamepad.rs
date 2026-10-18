//! Controllers for the prototype player's key/gamepad triggers: the buttons held, as bits at their
//! standard gamepad layout indices (`figcraft_doc::proto::GAMEPAD_BUTTONS`).
//! Windows reads Windows.Gaming.Input (Xbox-compatible controllers, through the `windows` crate the
//! app already uses). macOS, Linux and FreeBSD report none yet: the usual Rust controller library
//! links the system's libudev on Linux, which the pure-Rust builds avoid.

/// The standard layout's buttons held on any controller (bit i: index i).
#[cfg(target_os = "windows")]
pub fn held() -> u32 {
    use windows::Gaming::Input::{Gamepad, GamepadButtons as B};
    // The standard index of each button Windows reports; the triggers are analog (down past halfway).
    const MAP: [(B, u32); 14] = [
        (B::A, 0),
        (B::B, 1),
        (B::X, 2),
        (B::Y, 3),
        (B::LeftShoulder, 4),
        (B::RightShoulder, 5),
        (B::View, 8),
        (B::Menu, 9),
        (B::LeftThumbstick, 10),
        (B::RightThumbstick, 11),
        (B::DPadUp, 12),
        (B::DPadDown, 13),
        (B::DPadLeft, 14),
        (B::DPadRight, 15),
    ];
    let Ok(pads) = Gamepad::Gamepads() else { return 0 };
    let mut held = 0;
    for pad in pads {
        let Ok(r) = pad.GetCurrentReading() else { continue };
        for (b, i) in MAP {
            if r.Buttons.contains(b) {
                held |= 1 << i;
            }
        }
        if r.LeftTrigger > 0.5 {
            held |= 1 << 6;
        }
        if r.RightTrigger > 0.5 {
            held |= 1 << 7;
        }
    }
    held
}

#[cfg(not(target_os = "windows"))]
pub fn held() -> u32 {
    0
}
