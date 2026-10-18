//! The last line of defence: a panic inside a command becomes an [`EngineError::Internal`] and
//! the caller rolls the document back. It exists for bugs and doesn't replace `Result`.

use crate::{EngineError, Result};

pub fn run<T>(cmd: &str, f: impl FnOnce() -> Result<T>) -> Result<T> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(r) => r,
        Err(payload) => {
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "panic".to_string());
            log::error!("command `{cmd}` panicked: {msg}");
            Err(EngineError::Internal { cmd: cmd.to_string(), msg })
        }
    }
}
