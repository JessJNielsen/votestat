use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn connect() -> anyhow::Result<DatabaseConnection> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // TODO: How to ensure db exists?
    // if !Sqlite::database_exists(&db_url).await? {
    //     Sqlite::create_database(&db_url).await?;
    // }

    let db: DatabaseConnection = Database::connect(db_url).await?;

    Ok(db)
}
