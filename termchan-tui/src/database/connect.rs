use std::env;

use dotenv::dotenv;
use eyre::{bail, Error, Result};
use migration::{
    async_trait::{self, async_trait},
    DbErr,
    Migrator,
    MigratorTrait,
};
use sea_orm::{Database, DbConn};

#[cfg(target_os = "linux")]
const DATABASE_URL: &str = "sqlite:///var/tmp/termchan.db?mode=rwc";

#[cfg(target_os = "macos")]
const DATABASE_URL: &str = "sqlite:///var/tmp/termchan.db?mode=rwc";

#[cfg(target_os = "windows")]
const DATABASE_URL: &str = "sqlite:///C:\\Windows\\Temp\\termchan.db?mode=rwc";

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();

    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}
