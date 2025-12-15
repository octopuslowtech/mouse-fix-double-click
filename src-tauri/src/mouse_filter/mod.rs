use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
type PlatformHook = macos::MacMouseHook;
#[cfg(not(target_os = "macos"))]
mod unsupported;
#[cfg(not(target_os = "macos"))]
type PlatformHook = unsupported::UnsupportedHook;

type SharedService = Lazy<Mutex<MouseFilterService>>;

static SERVICE: SharedService = Lazy::new(|| Mutex::new(MouseFilterService::default()));

#[derive(Debug, thiserror::Error, Clone)]
pub enum FilterError {
    #[error("filter already running")]
    AlreadyRunning,
    #[error("filter not running")]
    NotRunning,
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    #[error("platform does not support global hooks")]
    Unsupported,
    #[error("service unavailable")]
    ServiceUnavailable,
    #[error("platform error: {0}")]
    Platform(String),
}

#[derive(Serialize)]
#[serde(tag = "name", content = "message")]
#[serde(rename_all = "camelCase")]
enum FilterErrorName {
    AlreadyRunning,
    NotRunning,
    Unsupported,
    ServiceUnavailable,
    Platform(String),
}

impl Serialize for FilterError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let name = match self {
            Self::AlreadyRunning => FilterErrorName::AlreadyRunning,
            Self::NotRunning => FilterErrorName::NotRunning,
            Self::Unsupported => FilterErrorName::Unsupported,
            Self::ServiceUnavailable => FilterErrorName::ServiceUnavailable,
            Self::Platform(message) => FilterErrorName::Platform(message.clone()),
        };
        name.serialize(serializer)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FilterStatus {
    pub running: bool,
    pub threshold_ms: u64,
    pub blocked_clicks: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockedEventPayload {
    pub delta_ms: u64,
}

pub fn start(handle: AppHandle, threshold_ms: u64) -> Result<FilterStatus, FilterError> {
    let mut service = SERVICE.lock().map_err(|_| FilterError::ServiceUnavailable)?;
    service.attach_handle(handle);
    service.start(threshold_ms)?;
    let snapshot = service.snapshot();
    service.emit_status();
    Ok(snapshot)
}

pub fn stop(handle: AppHandle) -> Result<FilterStatus, FilterError> {
    let mut service = SERVICE.lock().map_err(|_| FilterError::ServiceUnavailable)?;
    service.attach_handle(handle);
    service.stop()?;
    let snapshot = service.snapshot();
    service.emit_status();
    Ok(snapshot)
}

pub fn update_threshold(threshold_ms: u64) -> Result<FilterStatus, FilterError> {
    let mut service = SERVICE.lock().map_err(|_| FilterError::ServiceUnavailable)?;
    service.update_threshold(threshold_ms)?;
    let snapshot = service.snapshot();
    service.emit_status();
    Ok(snapshot)
}

pub fn status() -> FilterStatus {
    if let Ok(service) = SERVICE.lock() {
        service.snapshot()
    } else {
        FilterStatus {
            running: false,
            threshold_ms: 100,
            blocked_clicks: 0,
        }
    }
}

struct MouseFilterService {
    handle: Option<AppHandle>,
    hook: Option<PlatformHook>,
    running: bool,
    threshold_ms: u64,
    blocked_clicks: u64,
}

impl Default for MouseFilterService {
    fn default() -> Self {
        Self {
            handle: None,
            hook: None,
            running: false,
            threshold_ms: 100,
            blocked_clicks: 0,
        }
    }
}

impl MouseFilterService {
    fn attach_handle(&mut self, handle: AppHandle) {
        self.handle = Some(handle);
    }

    fn start(&mut self, threshold_ms: u64) -> Result<(), FilterError> {
        if self.running {
            if let Some(hook) = self.hook.as_mut() {
                hook.set_threshold(threshold_ms)?;
            }
            self.threshold_ms = threshold_ms;
            return Ok(());
        }
        let handle = self.handle.clone().ok_or(FilterError::ServiceUnavailable)?;
        let emitter = EventEmitter::new(handle.clone());
        let mut hook = PlatformHook::new(handle, emitter)?;
        hook.start(threshold_ms)?;
        self.threshold_ms = threshold_ms;
        self.running = true;
        self.blocked_clicks = 0;
        self.hook = Some(hook);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), FilterError> {
        if !self.running {
            return Err(FilterError::NotRunning);
        }
        if let Some(mut hook) = self.hook.take() {
            hook.stop()?;
            self.blocked_clicks = hook.blocked_clicks();
        }
        self.running = false;
        Ok(())
    }

    fn update_threshold(&mut self, threshold_ms: u64) -> Result<(), FilterError> {
        self.threshold_ms = threshold_ms;
        if self.running {
            if let Some(hook) = self.hook.as_mut() {
                hook.set_threshold(threshold_ms)?;
            }
        }
        Ok(())
    }

    fn snapshot(&self) -> FilterStatus {
        let blocked = if self.running {
            self.hook
                .as_ref()
                .map(|hook| hook.blocked_clicks())
                .unwrap_or(self.blocked_clicks)
        } else {
            self.blocked_clicks
        };
        FilterStatus {
            running: self.running,
            threshold_ms: self.threshold_ms,
            blocked_clicks: blocked,
        }
    }

    fn emit_status(&self) {
        if let Some(handle) = &self.handle {
            let _ = handle.emit("filter_status_changed", self.snapshot());
        }
    }

}

pub trait HookControl {
    fn new(handle: AppHandle, emitter: EventEmitter) -> Result<Self, FilterError>
    where
        Self: Sized;
    fn start(&mut self, threshold_ms: u64) -> Result<(), FilterError>;
    fn stop(&mut self) -> Result<(), FilterError>;
    fn set_threshold(&mut self, threshold_ms: u64) -> Result<(), FilterError>;
    fn blocked_clicks(&self) -> u64;
}

#[derive(Clone)]
pub struct EventEmitter {
    handle: AppHandle,
}

impl EventEmitter {
    fn new(handle: AppHandle) -> Self {
        Self { handle }
    }

    pub fn emit_blocked(&self, delta_ms: u64) {
        let _ = self
            .handle
            .emit("click_blocked", BlockedEventPayload { delta_ms });
    }
}

