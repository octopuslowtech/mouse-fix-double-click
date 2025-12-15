use super::{EventEmitter, FilterError, HookControl};

pub struct UnsupportedHook;

impl HookControl for UnsupportedHook {
    fn new(_handle: tauri::AppHandle, _emitter: EventEmitter) -> Result<Self, FilterError>
    where
        Self: Sized,
    {
        Err(FilterError::Unsupported)
    }

    fn start(&mut self, _threshold_ms: u64) -> Result<(), FilterError> {
        Err(FilterError::Unsupported)
    }

    fn stop(&mut self) -> Result<(), FilterError> {
        Err(FilterError::Unsupported)
    }

    fn set_threshold(&mut self, _threshold_ms: u64) -> Result<(), FilterError> {
        Err(FilterError::Unsupported)
    }

    fn blocked_clicks(&self) -> u64 {
        0
    }
}

