use crate::mouse_filter::{self, FilterStatus};
use tauri::AppHandle;

#[tauri::command]
pub fn start_filter(handle: AppHandle, threshold_ms: u64) -> Result<FilterStatus, crate::mouse_filter::FilterError> {
    mouse_filter::start(handle, threshold_ms)
}

#[tauri::command]
pub fn stop_filter(handle: AppHandle) -> Result<FilterStatus, crate::mouse_filter::FilterError> {
    mouse_filter::stop(handle)
}

#[tauri::command]
pub fn update_threshold(threshold_ms: u64) -> Result<FilterStatus, crate::mouse_filter::FilterError> {
    mouse_filter::update_threshold(threshold_ms)
}

#[tauri::command]
pub fn get_filter_status() -> FilterStatus {
    mouse_filter::status()
}

