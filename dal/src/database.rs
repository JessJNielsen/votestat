use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Executor, Pool, Sqlite, Transaction};
use std::env;
use std::time::Duration;

pub type DB = Pool<Sqlite>;
pub trait Queryer<'c>: Executor<'c, Database = Sqlite> {}

impl<'c> Queryer<'c> for &Pool<Sqlite> {}
impl<'c> Queryer<'c> for &'c mut Transaction<'_, Sqlite> {}

pub async fn connect() -> anyhow::Result<DB> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    if !Sqlite::database_exists(&db_url).await? {
        Sqlite::create_database(&db_url).await?;
    }

    SqlitePoolOptions::new()
        .max_connections(15)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(&db_url)
        .await
        .map_err(|err| {
            // error!("db: connecting to DB: {}", err);
            err.into()
        })
}

pub async fn migrate(database: &DB) -> anyhow::Result<()> {
    match sqlx::migrate!("./migrations").run(database).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }?;

    Ok(())
}
