use std::error::{Error};
use scraper::{Html, Selector};
use crate::models::{District, SuperDistrictEntity, SuperDistrictInput};
use crate::scraping::download::download;

pub async fn scrape() -> Result<(), Box<dyn Error>> {
    println!("Initializing scraping for KMD data.. \n");

    let super_districts: Vec<SuperDistrictInput> = scrape_main_page().await;

    for super_district in super_districts.iter() {
        SuperDistrictEntity::insert(super_district)
            .await
            .expect("Failed to insert Super District");

        // for sub_district in super_district.sub_districts {
        //
        // }
    }

    println!("Created {} new super districts", super_districts.len());

    Ok(())
}

async fn scrape_main_page() -> Vec<SuperDistrictInput> {
    println!("Scraping KMD Main Page");

    let result = download("https://kmdvalg.dk/Main")
        .await
        .expect("Failed to scrape KMD main page");

    parse_main_page(result.as_str())
}

fn parse_main_page(document: &str) -> Vec<SuperDistrictInput> {
    let parsed_html = Html::parse_document(document);

    // Grab Super Districts list from front page
    let district_lists_selector = Selector::parse("body .kmd-list-items .list-group").unwrap();
    let district_lists = parsed_html.select(&district_lists_selector);

    // Map each list to SuperDistrict -> District strcuts
    let selector_title = Selector::parse("div.list-group-item").expect("Failed to parse selector");
    let selector_href = Selector::parse("a.list-group-item").expect("Failed to parse selector");

    district_lists
        .into_iter()
        .map(|element| {
            let element_title = element
                .select(&selector_title)
                .next()
                .and_then(|n| n.text().next())
                .unwrap_or("Title not found");

            let element_links = element
                .select(&selector_href);

            SuperDistrictInput {
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
        .collect::<Vec<SuperDistrictInput>>()
}