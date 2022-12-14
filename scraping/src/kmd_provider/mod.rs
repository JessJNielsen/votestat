extern crate utils;

use async_trait::async_trait;

use chrono::naive::NaiveDate;
use domain::elections::entity::ElectionType;
use domain::elections::NewElectionDTO;
use domain::voting_districts::entity::VotingDistrictType;
use domain::voting_districts::{NewVotingDistrictDTO, VotingDistrict};
use scraper::{Html, Selector};
use utils::Context;

use crate::download::download;
use crate::ScrapingProvider;

pub struct KMDProvider;

#[derive(Debug, Clone)]
struct ScrapedDistrictReference {
    name: String,
    url: String,
}

#[derive(Debug, Clone)]
struct ScrapedSuperDistrict {
    name: String,
    districts: Vec<ScrapedDistrictReference>,
}

#[async_trait]
impl ScrapingProvider for KMDProvider {
    async fn scrape(&self, context: &Context) -> anyhow::Result<()> {
        println!("Initializing scraper for KMD data source. \n");
        println!("Scraping latest election data. \n");

        let super_districts: Vec<ScrapedSuperDistrict> = scrape_main_page().await;

        let election = context
            .election_service
            .create_election(NewElectionDTO {
                r#type: ElectionType::Parliament,
                date: NaiveDate::from_ymd_opt(2022, 11, 1).unwrap(),
            })
            .await?;

        println!("Created {:?} new election", election);

        // TODO: Parallelize with https://github.com/nikomatsakis/rayon
        for scraped_super_district in super_districts.iter() {
            let super_district = context
                .voting_district_service
                .create_voting_district(NewVotingDistrictDTO {
                    r#type: VotingDistrictType::Super,
                    parent_district_id: None,
                    name: "".to_string(),
                })
                .await
                .expect("Failed to insert Super District");

            println!("super_district {:?}", super_district);

            // for district in super_district.districts.iter() {}
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
                    .map(|link| ScrapedDistrictReference {
                        name: link.text().next().unwrap().trim().to_string(),
                        url: link.value().attr("href").unwrap().to_string(),
                    })
                    .collect::<Vec<ScrapedDistrictReference>>(),
            }
        })
        .collect::<Vec<ScrapedSuperDistrict>>()
}
