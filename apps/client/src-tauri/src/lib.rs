use anyhow::Context;
use tauri::Manager;

mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(mobile)]
            app.handle()
                .plugin(tauri_plugin_biometric::Builder::new().build());

            let connection = tauri::async_runtime::block_on(db::init())
                .context("failed to initialize database connection")?;

            app.manage(connection);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
