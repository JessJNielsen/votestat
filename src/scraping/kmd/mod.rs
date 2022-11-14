use std::error::{Error};
use scraper::{Html, Selector};
use crate::models::{District, SuperDistrict};
use crate::scraping::download::download;

pub async fn scrape() -> Result<(), Box<dyn Error>> {
    println!("Initializing scraping for KMD data.. \n");

    scrape_main_page().await
}

async fn scrape_main_page() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scraping KMD Main Page");

    let result = download("https://kmdvalg.dk/Main").await?;

    let document: &str = result.as_str();

    let list = parse_main_page(document);

    println!("links: {:#?}", list);

    Ok(())
}

fn parse_main_page(document: &str) -> Vec<SuperDistrict> {
    let parsed_html = Html::parse_document(document);

    // Grab Super Districts list from front page
    let district_lists_selector = Selector::parse("body .kmd-list-items .list-group").unwrap();
    let district_lists = parsed_html.select(&district_lists_selector).collect::<Vec<_>>();

    // Map each list to SuperDistrict -> District strcuts
    let selector_title = Selector::parse("div.list-group-item").unwrap();
    let selector_href = Selector::parse("a.list-group-item").unwrap();

    district_lists
        .into_iter()
        .map(|element| {
            let element_title = element
                .select(&selector_title)
                .next()
                .and_then(|n| n.text().nth(0))
                .unwrap_or("Title not found");

            let element_links = element
                .select(&selector_href)
                .collect::<Vec<_>>();

            SuperDistrict {
                name: element_title.to_string(),
                sub_districts: element_links
                    .into_iter()
                    .map(|link| {
                        District {
                            name: link.text().next().unwrap().trim().to_string(),
                            link: link.value().attr("href").unwrap().to_string()
                        }
                    }).collect::<Vec<District>>()
            }
        })
        .collect::<Vec<SuperDistrict>>()
}