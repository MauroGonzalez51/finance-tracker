use anyhow::Context;
use i18n::I18nState;
use tauri::{Manager, WindowEvent};

mod db;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            #[cfg(mobile)]
            app.handle()
                .plugin(tauri_plugin_biometric::Builder::new().build());

            let i18n_state = I18nState::default();
            app.manage(i18n_state);

            app.handle().plugin(tauri_plugin_notification::init())?;

            tray::setup(app.handle())?;

            let connection = tauri::async_runtime::block_on(db::init())
                .context("failed to initialize database connection")?;

            app.manage(connection);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
