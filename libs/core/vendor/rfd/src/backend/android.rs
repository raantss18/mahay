//! Android backend (added by Mahay; not in upstream rfd).
//!
//! Open dialogs use the Storage Access Framework through `mahay-android`: the picked document is
//! copied into the app's storage and its path returned; writes to that copy are copied back to
//! the original by `mahay_android::poll`. Save dialogs return a path in the app's storage at once,
//! and `mahay_android::poll` asks where to put the document after the app writes it. There is no
//! folder picker yet (folders under SAF are trees of URIs, not paths): folder dialogs answer
//! "cancelled". Message dialogs aren't shown; they log and answer the default.
//!
//! The futures resolve on a waiting thread; the blocking (sync) variants must not be called from
//! the thread running the app.

use crate::backend::{
    AsyncFilePickerDialogImpl, AsyncFileSaveDialogImpl, AsyncFolderPickerDialogImpl, AsyncMessageDialogImpl, DialogFutureType,
    FilePickerDialogImpl, FileSaveDialogImpl, FolderPickerDialogImpl, MessageDialogImpl,
};
use crate::message_dialog::{MessageButtons, MessageDialog, MessageDialogResult};
use crate::{FileDialog, FileHandle};
use std::path::PathBuf;

fn mimes(dialog: &FileDialog) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for filter in &dialog.filters {
        for ext in &filter.extensions {
            let ext = ext.trim_start_matches("*.").trim_start_matches('.');
            if ext == "*" || ext.is_empty() {
                return Vec::new();
            }
            let mime = mahay_android::mime_for(&format!("x.{ext}"));
            if mime == "application/octet-stream" {
                // An extension Android has no type for: show everything rather than nothing.
                return Vec::new();
            }
            if !out.contains(&mime) {
                out.push(mime);
            }
        }
    }
    out
}

fn pick(dialog: &FileDialog) -> Option<PathBuf> {
    let mimes = mimes(dialog);
    mahay_android::pick_document(&mimes).recv().ok().flatten().map(|p| p.path)
}

fn save_path(dialog: &FileDialog) -> PathBuf {
    let name = dialog.file_name.clone().unwrap_or_else(|| "Document".into());
    mahay_android::save_target(&name)
}

impl FilePickerDialogImpl for FileDialog {
    fn pick_file(self) -> Option<PathBuf> {
        pick(&self)
    }
    fn pick_files(self) -> Option<Vec<PathBuf>> {
        pick(&self).map(|p| vec![p])
    }
}

impl AsyncFilePickerDialogImpl for FileDialog {
    fn pick_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let rx = mahay_android::pick_document(&mimes(&self));
        Box::pin(Waiting::new(move || rx.recv().ok().flatten().map(|p| FileHandle::wrap(p.path))))
    }
    fn pick_files_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        let rx = mahay_android::pick_document(&mimes(&self));
        Box::pin(Waiting::new(move || rx.recv().ok().flatten().map(|p| vec![FileHandle::wrap(p.path)])))
    }
}

impl FileSaveDialogImpl for FileDialog {
    fn save_file(self) -> Option<PathBuf> {
        Some(save_path(&self))
    }
}

impl AsyncFileSaveDialogImpl for FileDialog {
    fn save_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let path = save_path(&self);
        Box::pin(std::future::ready(Some(FileHandle::wrap(path))))
    }
}

impl FolderPickerDialogImpl for FileDialog {
    fn pick_folder(self) -> Option<PathBuf> {
        log::warn!("rfd: folder pickers aren't available on Android yet");
        None
    }
    fn pick_folders(self) -> Option<Vec<PathBuf>> {
        self.pick_folder().map(|p| vec![p])
    }
}

impl AsyncFolderPickerDialogImpl for FileDialog {
    fn pick_folder_async(self) -> DialogFutureType<Option<FileHandle>> {
        Box::pin(std::future::ready(FolderPickerDialogImpl::pick_folder(self).map(FileHandle::wrap)))
    }
    fn pick_folders_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        Box::pin(std::future::ready(None))
    }
}

fn default_answer(dialog: &MessageDialog) -> MessageDialogResult {
    log::info!("rfd message (not shown on Android): {}: {}", dialog.title, dialog.description);
    match &dialog.buttons {
        MessageButtons::Ok | MessageButtons::OkCustom(_) => MessageDialogResult::Ok,
        _ => MessageDialogResult::Cancel,
    }
}

impl MessageDialogImpl for MessageDialog {
    fn show(self) -> MessageDialogResult {
        default_answer(&self)
    }
}

impl AsyncMessageDialogImpl for MessageDialog {
    fn show_async(self) -> DialogFutureType<MessageDialogResult> {
        Box::pin(std::future::ready(default_answer(&self)))
    }
}

/// A future that runs a blocking wait on its own thread.
struct Waiting<T> {
    state: std::sync::Arc<std::sync::Mutex<(Option<T>, Option<std::task::Waker>)>>,
}

impl<T: Send + 'static> Waiting<T> {
    fn new(wait: impl FnOnce() -> T + Send + 'static) -> Self {
        let state: std::sync::Arc<std::sync::Mutex<(Option<T>, Option<std::task::Waker>)>> = Default::default();
        let shared = state.clone();
        let spawned = std::thread::Builder::new().name("rfd_android_pick".into()).spawn(move || {
            let value = wait();
            let mut s = shared.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            s.0 = Some(value);
            if let Some(w) = s.1.take() {
                w.wake();
            }
        });
        if let Err(e) = spawned {
            log::error!("rfd: couldn't start the picker thread: {e}");
        }
        Self { state }
    }
}

impl<T> std::future::Future for Waiting<T> {
    type Output = T;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<T> {
        let mut s = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match s.0.take() {
            Some(v) => std::task::Poll::Ready(v),
            None => {
                s.1 = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        }
    }
}
