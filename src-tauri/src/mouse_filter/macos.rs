use super::{EventEmitter, FilterError, HookControl};
use core_foundation::base::{Boolean, TCFType};
use core_foundation::mach_port::{CFMachPort, CFMachPortRef};
use core_foundation::runloop::{
    kCFRunLoopDefaultMode, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRunInMode,
    CFRunLoopSource,
};
use core_graphics::event::{
    CGEventMask, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventTapProxy,
    CGEventType,
};
use core_graphics::sys::CGEventRef;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::process::Command;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: CGEventMask,
        callback: CGEventTapCallback,
        user_info: *mut c_void,
    ) -> CFMachPortRef;

    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);

    fn AXIsProcessTrusted() -> Boolean;
}

type CGEventTapCallback = unsafe extern "C" fn(
    CGEventTapProxy,
    CGEventType,
    CGEventRef,
    *mut c_void,
) -> CGEventRef;

pub struct MacMouseHook {
    state: Arc<HookState>,
    worker: Option<JoinHandle<()>>,
    stopper: Option<Sender<()>>,
}

struct HookState {
    threshold_ms: AtomicU64,
    blocked: AtomicU64,
    last_click: Mutex<Option<Instant>>,
    emitter: EventEmitter,
}

impl HookState {
    fn should_block(&self, event_type: CGEventType) -> Option<u64> {
        let ty = event_type as u32;
        let left = CGEventType::LeftMouseDown as u32;
        let right = CGEventType::RightMouseDown as u32;
        let other = CGEventType::OtherMouseDown as u32;
        if ty != left && ty != right && ty != other {
            return None;
        }
        let now = Instant::now();
        let mut guard = self.last_click.lock().ok()?;
        let threshold = self.threshold_ms.load(Ordering::Relaxed);
        if let Some(prev) = *guard {
            let delta = now.duration_since(prev);
            let delta_ms = delta.as_micros() as u64 / 1000;
            if delta_ms < threshold {
                self.blocked.fetch_add(1, Ordering::Relaxed);
                return Some(delta_ms);
            }
        }
        *guard = Some(now);
        None
    }
}

impl HookControl for MacMouseHook {
    fn new(_handle: tauri::AppHandle, emitter: EventEmitter) -> Result<Self, FilterError> {
        Ok(Self {
            state: Arc::new(HookState {
                threshold_ms: AtomicU64::new(100),
                blocked: AtomicU64::new(0),
                last_click: Mutex::new(None),
                emitter,
            }),
            worker: None,
            stopper: None,
        })
    }

    fn start(&mut self, threshold_ms: u64) -> Result<(), FilterError> {
        if self.worker.is_some() {
            return Err(FilterError::AlreadyRunning);
        }
        if !request_accessibility_permission() {
            return Err(FilterError::Platform(
                "macOS Accessibility permission required. Enable in System Settings > Privacy & Security > Accessibility, then relaunch."
                    .into(),
            ));
        }
        self.state
            .threshold_ms
            .store(threshold_ms, Ordering::Relaxed);
        let (tx_stop, rx_stop) = channel();
        let (tx_ready, rx_ready) = channel();
        let state = self.state.clone();
        let worker = thread::spawn(move || run_loop(state, rx_stop, tx_ready));
        let ready = rx_ready
            .recv_timeout(Duration::from_secs(2))
            .unwrap_or(false);
        if ready {
            self.worker = Some(worker);
            self.stopper = Some(tx_stop);
            Ok(())
        } else {
            let _ = tx_stop.send(());
            let _ = worker.join();
            Err(FilterError::Platform(
                "unable to initialize macOS mouse hook".into(),
            ))
        }
    }

    fn stop(&mut self) -> Result<(), FilterError> {
        if let Some(stop) = self.stopper.take() {
            let _ = stop.send(());
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        Ok(())
    }

    fn set_threshold(&mut self, threshold_ms: u64) -> Result<(), FilterError> {
        self.state
            .threshold_ms
            .store(threshold_ms, Ordering::Relaxed);
        Ok(())
    }

    fn blocked_clicks(&self) -> u64 {
        self.state.blocked.load(Ordering::Relaxed)
    }
}

fn run_loop(state: Arc<HookState>, stop: Receiver<()>, ready: Sender<bool>) {
    unsafe extern "C" fn tap_callback(
        _proxy: CGEventTapProxy,
        event_type: CGEventType,
        event: CGEventRef,
        user_data: *mut c_void,
    ) -> CGEventRef {
        let state = &*(user_data as *const HookState);
        if let Some(delta) = state.should_block(event_type) {
            state.emitter.emit_blocked(delta);
            return null_mut();
        }
        event
    }

    let mask = build_event_mask();
    let user_data = Arc::into_raw(state.clone()) as *mut c_void;
    let tap_ref = unsafe {
        CGEventTapCreate(
            CGEventTapLocation::HID,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            mask,
            tap_callback,
            user_data,
        )
    };
    if tap_ref.is_null() {
        let _ = ready.send(false);
        unsafe { Arc::from_raw(user_data as *const HookState) };
        return;
    }
    let mach_port = unsafe { CFMachPort::wrap_under_create_rule(tap_ref) };
    let source: CFRunLoopSource = match mach_port.create_runloop_source(0) {
        Ok(source) => source,
        Err(_) => {
            let _ = ready.send(false);
            unsafe { Arc::from_raw(user_data as *const HookState) };
            return;
        }
    };
    let run_loop = unsafe { CFRunLoopGetCurrent() };
    unsafe {
        CFRunLoopAddSource(run_loop, source.as_concrete_TypeRef(), kCFRunLoopDefaultMode);
        CGEventTapEnable(mach_port.as_concrete_TypeRef(), true);
    }
    let _ = ready.send(true);
    loop {
        unsafe {
            CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.1, 1 as Boolean);
        }
        if stop.try_recv().is_ok() {
            break;
        }
    }
    unsafe {
        CGEventTapEnable(mach_port.as_concrete_TypeRef(), false);
    }
    unsafe { Arc::from_raw(user_data as *const HookState) };
}

fn build_event_mask() -> CGEventMask {
    let left = 1u64 << (CGEventType::LeftMouseDown as u64);
    let right = 1u64 << (CGEventType::RightMouseDown as u64);
    let other = 1u64 << (CGEventType::OtherMouseDown as u64);
    left | right | other
}

fn request_accessibility_permission() -> bool {
    let trusted = unsafe { AXIsProcessTrusted() != 0 };
    if trusted {
        return true;
    }
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn();
    false
}

