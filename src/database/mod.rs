use sqlx::migrate::{MigrateDatabase};
use sqlx::{migrate, Pool, Sqlite, SqlitePool};

const DB_PATH: &'static str = "./storage/votestat.db";

/// Connects to the database. If no database exists, one is created.
///
/// Then it runs all migrations in ./migrations.
pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    // Create the database if not exists
    let db_url = db_url();

    if !Sqlite::database_exists(&db_url).await? {
        Sqlite::create_database(&db_url).await?;
    }

    // Connect to the database
    let sqlite_pool = connect().await;

    // Migrate the database
    migrate!().run(&sqlite_pool).await?;

    Ok(())
}

fn db_url() -> String {
    format!("sqlite:{}", DB_PATH)
}

async fn connect() -> Pool<Sqlite> {
    SqlitePool::connect(&db_url()).await.unwrap()
}

// Plan

// Table and entities to make

// Party
    // has id and name of party

// Super District
    // has id and name

// District
    // has id and name and id of super district

// Voting Center
    // has id and name and id of district

// Candidate
    // has id and name and id of party

// IndependentCandidate
    // Same as a party but only in one super district

// PartyVotesVotingCentre
    // Each row has id of party and of voting centre and amount of votes

// CandidateVotesVotingCentre
    // Each row has id of candidate and of voting centre and amount of votes

