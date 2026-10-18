//! Android backend (added by Mahay; not in upstream arboard): plain text through the system
//! clipboard via `mahay-android`. HTML is written as its plain-text alternative; images and file
//! lists aren't supported yet.

use crate::common::Error;
#[cfg(feature = "image-data")]
use crate::common::ImageData;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub(crate) struct Clipboard;

impl Clipboard {
	pub(crate) fn new() -> Result<Clipboard, Error> {
		Ok(Clipboard)
	}
}

pub(crate) struct Get<'clipboard> {
	_clipboard: &'clipboard mut Clipboard,
}

impl<'clipboard> Get<'clipboard> {
	pub(crate) fn new(clipboard: &'clipboard mut Clipboard) -> Self {
		Self { _clipboard: clipboard }
	}

	pub(crate) fn text(self) -> Result<String, Error> {
		mahay_android::clipboard_text().ok_or(Error::ContentNotAvailable)
	}

	pub(crate) fn html(self) -> Result<String, Error> {
		Err(Error::ContentNotAvailable)
	}

	#[cfg(feature = "image-data")]
	pub(crate) fn image(self) -> Result<ImageData<'static>, Error> {
		Err(Error::ContentNotAvailable)
	}

	pub(crate) fn file_list(self) -> Result<Vec<PathBuf>, Error> {
		Err(Error::ContentNotAvailable)
	}
}

pub(crate) struct Set<'clipboard> {
	_clipboard: &'clipboard mut Clipboard,
}

impl<'clipboard> Set<'clipboard> {
	pub(crate) fn new(clipboard: &'clipboard mut Clipboard) -> Self {
		Self { _clipboard: clipboard }
	}

	pub(crate) fn text(self, data: Cow<'_, str>) -> Result<(), Error> {
		mahay_android::set_clipboard_text(&data);
		Ok(())
	}

	pub(crate) fn html(self, html: Cow<'_, str>, alt: Option<Cow<'_, str>>) -> Result<(), Error> {
		mahay_android::set_clipboard_text(alt.as_deref().unwrap_or(&html));
		Ok(())
	}

	#[cfg(feature = "image-data")]
	pub(crate) fn image(self, _data: ImageData) -> Result<(), Error> {
		Err(Error::ClipboardNotSupported)
	}

	pub(crate) fn file_list(self, _file_list: &[impl AsRef<Path>]) -> Result<(), Error> {
		Err(Error::ClipboardNotSupported)
	}
}

pub(crate) struct Clear<'clipboard> {
	_clipboard: &'clipboard mut Clipboard,
}

impl<'clipboard> Clear<'clipboard> {
	pub(crate) fn new(clipboard: &'clipboard mut Clipboard) -> Self {
		Self { _clipboard: clipboard }
	}

	pub(crate) fn clear(self) -> Result<(), Error> {
		mahay_android::set_clipboard_text("");
		Ok(())
	}
}
