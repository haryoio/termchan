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

#[async_trait::async_trait]
pub trait Repository {
    type From;
    type To;
    async fn get(from: Self::From) -> Result<Self::To>;
    async fn update(&mut self, to: Self::To) -> Result<()>;
}

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}
