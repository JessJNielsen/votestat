use std::env;
use sqlx::migrate::{MigrateDatabase};
use sqlx::{migrate, Pool, Sqlite, SqlitePool};

/// Initialized the database. If no database exists, one is created.
///
/// Then it runs all migrations in ./migrations.
pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    // Create the database if not exists
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    if !Sqlite::database_exists(&db_url).await? {
        Sqlite::create_database(&db_url).await?;
    }

    // Connect to the database
    let sqlite_pool = connect_database().await;

    // Migrate the database
    migrate!().run(&sqlite_pool).await?;

    Ok(())
}

/// Connects to the database.
pub async fn connect_database() -> Pool<Sqlite> {
    SqlitePool::connect(
        env::var("DATABASE_URL").unwrap().as_str()
    ).await.unwrap()
}

