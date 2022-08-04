use sea_orm::Database;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
