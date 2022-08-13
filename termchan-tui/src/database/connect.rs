use eyre::{bail, Result};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

use crate::config::dirs::Dir;

pub async fn establish_connection() -> Result<DbConn> {
    let path = match Dir::get_db_path() {
        Ok(path) => path,
        Err(e) => {
            error!("{}", e);
            bail!(e);
        }
    };

    let db = Database::connect(path).await?;
    Migrator::up(&db, None).await?;

    info!("Database connection established");

    Ok(db)
}
