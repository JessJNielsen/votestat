extern crate utils;

use async_trait::async_trait;

use scraper::{Html, Selector};
use utils::Context;

use crate::download::download;
use crate::ScrapingProvider;

pub struct KMDProvider;

#[derive(Debug, Clone)]
struct ScrapedDistrict {
    name: String,
    url: String,
}

#[derive(Debug, Clone)]
struct ScrapedSuperDistrict {
    name: String,
    districts: Vec<ScrapedDistrict>,
}

#[async_trait]
impl ScrapingProvider for KMDProvider {
    async fn scrape(&self, context: &Context) -> anyhow::Result<()> {
        println!("Initializing scraping for KMD data.. \n");

        let super_districts: Vec<ScrapedSuperDistrict> = scrape_main_page().await;

        // TODO: Parallelize with https://github.com/nikomatsakis/rayon
        for super_district in super_districts.iter() {
            context
                .dal_service
                .create_super_district(&super_district.name)
                .await
                .expect("Failed to insert Super District");

            // for district in super_district.disctricts {
            //
            // }
        }

        println!("Created {} new super districts", super_districts.len());

        Ok(())
    }
}

async fn scrape_main_page() -> Vec<ScrapedSuperDistrict> {
    println!("Scraping KMD Main Page");

    let result = download("https://kmdvalg.dk/Main")
        .await
        .expect("Failed to scrape KMD main page");

    parse_main_page(result.as_str())
}

fn parse_main_page(document: &str) -> Vec<ScrapedSuperDistrict> {
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

            let element_links = element.select(&selector_href);

            ScrapedSuperDistrict {
                name: element_title.to_string(),
                districts: element_links
                    .into_iter()
                    .map(|link| ScrapedDistrict {
                        name: link.text().next().unwrap().trim().to_string(),
                        url: link.value().attr("href").unwrap().to_string(),
                    })
                    .collect::<Vec<ScrapedDistrict>>(),
            }
        })
        .collect::<Vec<ScrapedSuperDistrict>>()
}
