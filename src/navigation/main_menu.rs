use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum MainMenuOption {
    Scrape,
    Read,
    Analyze,
}

impl MainMenuOption {
    pub fn parse_selection(selection: &str) -> Result<MainMenuOption, ()> {
        MainMenuOption::from_str(selection)
    }
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