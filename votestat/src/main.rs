extern crate scraping;
extern crate utils;

use dotenvy::dotenv;
use inquire::Select;
use migration::{Migrator, MigratorTrait};
use navigation::main_menu::MainMenuOption;
use scraping::run_scraping_tools;
use std::env;
use utils::Context;

mod navigation;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");

    println!(
        r#"
VoteStat - Vote Statistics and Analysis

With this tool you can download data from KMD.
Then you can either do simple analysis or export it for more detailed analysis.
    "#
    );

    let votestat_context = initialize_db_and_context().await;

    let options: Vec<&str> = vec!["Scrape", "Read", "Export"];

    let selection = Select::new("To get started, please choose an option", options).prompt();

    let selected_main_menu_option = match selection {
        Ok(ans) => MainMenuOption::parse_selection(ans),
        Err(_) => Err(println!(
            "There was an error with your selection, please try again."
        )),
    };

    match selected_main_menu_option.unwrap() {
        MainMenuOption::Scrape => run_scraping_tools(votestat_context).await?,
        MainMenuOption::Read => println!("TODO: Implement Read"),
        MainMenuOption::Analyze => println!("TODO: Implement Read"),
    }

    Ok(())
}

/// Initializes the Database and any services
///
/// Returns a Context object
async fn initialize_db_and_context() -> Context {
    let db: database::DatabaseConnection = database::database::connect()
        .await
        .expect("Could not connect to Database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to migrate Database");

    // TODO: replace with shaku dependency injection setup for better decoupling
    Context {
        election_service: domain::elections::ElectionService::new(db.clone()),
        voting_district_service: domain::voting_districts::VotingDistrictService::new(db.clone()),
    }
}
