pub mod entity;

use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

fn get_connection_url() -> anyhow::Result<String> {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        return Ok(url);
    }

    let project_dirs = directories::ProjectDirs::from("com", "MauroGonzalez51", "FinanceTracker")
        .context("failed to get project dirs")?;

    let data_dir = project_dirs.data_dir();
    std::fs::create_dir_all(data_dir)
        .with_context(|| format!("failed to create directory: {}", data_dir.display()))?;

    let db_path = data_dir.join("dev.db");
    if !db_path.exists() {
        std::fs::File::create(&db_path)
            .with_context(|| format!("failed to create file: {}", db_path.display()))?;
    }

    Ok(format!("sqlite://{}?mode=rwc", db_path.display()))
}

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let url = get_connection_url().context("failed to get database url")?;
    let connection = Database::connect(&url).await?;

    Migrator::up(&connection, None).await?;

    Ok(connection)
}
