pub mod entity;

use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

fn get_connection_url() -> anyhow::Result<String> {
    if cfg!(debug_assertions) {
        return Ok(std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://dev.db?mode=rwc".to_string()));
    }

    let project_dirs = directories::ProjectDirs::from("com", "MauroGonzalez51", "FinanceTracker")
        .context("failed to get project dirs")?;

    let data_directory = project_dirs.data_dir();
    std::fs::create_dir_all(data_directory).with_context(|| {
        format!(
            "failed to create data directory: {}",
            data_directory.display()
        )
    })?;

    let db_path = data_directory.join("finance.db");
    Ok(format!("sqlite://{}?mode=rwc", db_path.display()))
}

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let url = get_connection_url().context("failed to get database url")?;
    let connection = Database::connect(&url).await?;

    Migrator::up(&connection, None).await?;

    Ok(connection)
}
