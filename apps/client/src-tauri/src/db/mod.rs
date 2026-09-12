pub mod entity;

use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

fn get_encryption_key() -> anyhow::Result<String> {
    let entry = keyring::Entry::new("FinanceTracker", "DB_ENCRYPTION_KEY")
        .context("failed to create keyring entry")?;

    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(_) => {
            let key = uuid::Uuid::new_v4().to_string();
            entry
                .set_password(&key)
                .context("failed to save key to keyring")?;

            Ok(key)
        }
    }
}

fn get_connection_url() -> anyhow::Result<String> {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        return Ok(url);
    }

    let project_dirs = directories::ProjectDirs::from("com", "MauroGonzalez51", "FinanceTracker")
        .context("failed to get project dirs")?;

    let data_dir = project_dirs.data_dir();
    std::fs::create_dir_all(data_dir)
        .with_context(|| format!("failed to create directory: {}", data_dir.display()))?;

    let db_path = data_dir.join("database.db");
    if !db_path.exists() {
        std::fs::File::create(&db_path)
            .with_context(|| format!("failed to create file: {}", db_path.display()))?;
    }

    Ok(format!("sqlite://{}?mode=rwc", db_path.display()))
}

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let url = get_connection_url().context("failed to get database url")?;
    let connection = Database::connect(&url).await?;

    let encryption_key = get_encryption_key()?;

    connection
        .execute_unprepared(&format!("PRAGMA key = '{}';", encryption_key))
        .await?;

    Migrator::up(&connection, None).await?;

    Ok(connection)
}
