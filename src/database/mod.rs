use rusqlite::{Connection};

const DB_PATH: &'static str = "./src/database/votestat.db";

pub async fn initialize_database() -> rusqlite::Result<Connection> {
    Connection::open(DB_PATH)
}