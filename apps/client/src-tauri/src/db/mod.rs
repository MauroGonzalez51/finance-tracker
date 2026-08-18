mod schema;

pub mod accounts;
pub mod categories;
pub mod payment_methods;
pub mod profiles;
pub mod transactions;

use std::path::PathBuf;

use anyhow::Context;
use diesel::sqlite::SqliteConnection;
use diesel::{Connection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn database_path() -> anyhow::Result<PathBuf> {
    if cfg!(debug_assertions) {
        return Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("database.db"));
    }

    let project_dirs = ProjectDirs::from_path(PathBuf::from("com.maurogonzalez51.finance-tracker"))
        .context("failed to resolve application data directory")?;

    let data_dir = project_dirs.data_dir();
    std::fs::create_dir_all(data_dir)
        .with_context(|| format!("failed to create data directory at {}", data_dir.display()))?;

    Ok(data_dir.join("database.db"))
}

pub fn establish_connection() -> anyhow::Result<SqliteConnection> {
    let path = database_path()?;
    let mut connection = SqliteConnection::establish(&path.to_string_lossy())
        .with_context(|| format!("failed to connect to database at {}", path.display()))?;

    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut connection)
        .context("failed to enable foreign key enforcement")?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|err| anyhow::anyhow!("failed to run pending migrations: {err}"))?;

    Ok(connection)
}
