mod db;

use diesel::sqlite::SqliteConnection;
use std::sync::Mutex;

pub struct DbConnection(pub Mutex<SqliteConnection>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let connection = db::establish_connection().expect("failed to set up database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(DbConnection(Mutex::new(connection)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
