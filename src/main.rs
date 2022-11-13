mod models;
mod database;
mod navigation;
mod scraping;

use inquire::{InquireError, Select};
use crate::navigation::main_menu::{MainMenuOption, parse_selection};
use crate::scraping::kmd::scrape;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\
        VoteStat - Vote Statistics and Analysis \n\n\
        With this tool you can download data from KMD. \n\
        Then you can either do simple analysis or export it for more detailed analysis.
    ");

    database::initialize_database()
        .await
        .expect("Failed to initialize database");


    let options: Vec<&str> = vec!["Scrape", "Read", "Export"];

    let selection: Result<&str, InquireError> = Select::new("To get started, please choose an option", options).prompt();

    let selected_main_menu_option = match selection {
        Ok(ans) => parse_selection(ans),
        Err(_) => Err(println!("There was an error with your selection, please try again.")),
    };

    match selected_main_menu_option.unwrap() {
        MainMenuOption::Scrape => scrape().await.expect("TODO: Message for failed scrape"),
        MainMenuOption::Read => println!("TODO: Implement Read"),
        MainMenuOption::Analyze => println!("TODO: Implement Read"),
    }

    Ok(())
}