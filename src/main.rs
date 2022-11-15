mod models;
mod database;
mod navigation;
mod scraping;

use dotenvy::dotenv;
use inquire::Select;
use crate::navigation::main_menu::{MainMenuOption};
use crate::scraping::kmd::scrape;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    println!(r#"
VoteStat - Vote Statistics and Analysis

With this tool you can download data from KMD.
Then you can either do simple analysis or export it for more detailed analysis.
    "#);

    database::initialize().await.expect("Could not connect and migrate DB");

    let options: Vec<&str> = vec!["Scrape", "Read", "Export"];

    let selection = Select::new("To get started, please choose an option", options).prompt();

    let selected_main_menu_option = match selection {
        Ok(ans) => MainMenuOption::parse_selection(ans),
        Err(_) => Err(println!("There was an error with your selection, please try again.")),
    };

    match selected_main_menu_option.unwrap() {
        MainMenuOption::Scrape => scrape().await.expect("TODO: Message for failed scrape"),
        MainMenuOption::Read => println!("TODO: Implement Read"),
        MainMenuOption::Analyze => println!("TODO: Implement Read"),
    }

    Ok(())
}