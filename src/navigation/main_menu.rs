use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum MainMenuOption {
    Scrape,
    Read,
    Analyze,
}

impl FromStr for MainMenuOption {
    type Err = ();

    fn from_str(input: &str) -> Result<MainMenuOption, Self::Err> {
        match input {
            "Scrape"  => Ok(MainMenuOption::Scrape),
            "Read"  => Ok(MainMenuOption::Read),
            "Export"  => Ok(MainMenuOption::Analyze),
            _      => Err(()),
        }
    }
}

pub fn parse_selection(selection: &str) -> Result<MainMenuOption, ()> {
    MainMenuOption::from_str(selection)
}