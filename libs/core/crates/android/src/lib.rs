//! Android glue shared by every Mahay app.
//!
//! The apps run in `org.mahay.shell.MahayActivity`, a small Java `GameActivity` subclass
//! (`android/shell/`). This crate reaches the parts of Android that Rust can't reach alone:
//!
//! - [`init`]: logcat logging, a panic hook that logs, and `HOME`/`XDG_*`/`TMPDIR` pointing at
//!   the app's private folders, so code written for Linux finds somewhere to keep its settings.
//! - [`open_document`] / [`save_target`] / [`poll`]: documents. Android hands out
//!   `content://` URIs, not paths, so the shell copies a picked document into the app's private
//!   storage and Rust edits that copy. [`poll`] copies each new write back to the original
//!   (`ACTION_OPEN_DOCUMENT` grants write access), and asks where to put a Save As
//!   (`ACTION_CREATE_DOCUMENT`) after its first write. The pickers never block the Rust thread:
//!   blocking it would deadlock the activity, whose Java side waits for the native thread
//!   while the picker covers the window.
//! - VIEW / SEND intents: the launch document comes out of the first [`poll`].
//! - [`launch_app`], [`is_installed`], [`open_url`]: what Mahay HUB needs to start sibling apps.
//! - [`set_keyboard`], [`clipboard_text`], [`set_clipboard_text`]: soft keyboard and clipboard.
//!
//! Answers from Java arrive on the UI thread; [`set_waker`] lets the app ask for a repaint then.
//! On other platforms the crate is empty.

#[cfg(target_os = "android")]
mod imp;

#[cfg(target_os = "android")]
pub use imp::*;

#[cfg(all(target_os = "android", feature = "egui"))]
mod bridge;

#[cfg(all(target_os = "android", feature = "egui"))]
pub use bridge::EguiBridge;
