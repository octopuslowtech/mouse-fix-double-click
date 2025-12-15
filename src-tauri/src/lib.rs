mod commands;
mod mouse_filter;
use commands::mouse::{get_filter_status, start_filter, stop_filter, update_threshold};
use commands::system::{get_autostart_enabled, set_autostart_enabled};
use tauri_plugin_autostart::MacosLauncher;
use tauri::{
    tray::{TrayIconBuilder, TrayIconEvent, TrayIcon},
    menu::{MenuBuilder, MenuItemBuilder, MenuEvent},
    Manager, WindowEvent, ActivationPolicy,
};

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_filter_status,
            start_filter,
            stop_filter,
            update_threshold,
            get_autostart_enabled,
            set_autostart_enabled
        ])
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None
        ))
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                #[cfg(target_os = "macos")]
                let _ = window.app_handle().set_activation_policy(ActivationPolicy::Accessory);
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .setup(|app| {
            #[cfg(target_os = "macos")]
            let _ = app.set_activation_policy(ActivationPolicy::Accessory);
            let app_handle = app.handle();
            let show_item = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let mut tray_menu = MenuBuilder::new(app);
            tray_menu = tray_menu.item(&show_item);
            tray_menu = tray_menu.item(&quit_item);
            let tray_menu = tray_menu.build()?;
            let _ = TrayIconBuilder::new()
                .icon(app_handle.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(|app, event: MenuEvent| match event.id().as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            #[cfg(target_os = "macos")]
                            let _ = app.set_activation_policy(ActivationPolicy::Regular);
                            let _ = win.show();
                            let _ = win.unminimize();
                            let _ = win.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app);
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
