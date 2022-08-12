use directories::ProjectDirs;
use migration::Migrator;
use sea_orm::Database;
use sea_orm_migration::prelude::*;

#[cfg(target_os = "linux")]
const DATABASE_URL: &str = "sqlite:///var/tmp/termchan.db?mode=rwc";

#[cfg(target_os = "macos")]
const DATABASE_URL: &str = "sqlite:///var/tmp/termchan.db?mode=rwc";

#[cfg(target_os = "windows")]
const DATABASE_URL: &str = "sqlite:///C:\\Windows\\Temp\\termchan.db?mode=rwc";

#[tokio::main]
async fn main() {
    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");
}
