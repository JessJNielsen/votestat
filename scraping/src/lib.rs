#![no_main]

use crate::kmd_provider::KMDProvider;
use async_trait::async_trait;
use utils::Context;

pub mod download;
pub mod kmd_provider;

pub async fn run_scraping_tools(votestat_context: Context) -> anyhow::Result<()> {
    // TODO: Switch between KMD and other providers

    Scraper::new(KMDProvider).scrape(&votestat_context).await?;

    Ok(())
}

// Provider traits and structs, technically a strategy pattern

#[async_trait]
trait ScrapingProvider {
    async fn scrape(&self, context: &Context) -> anyhow::Result<()>;
}

struct Scraper<T: ScrapingProvider> {
    provider: T,
}

impl<T: ScrapingProvider> Scraper<T> {
    pub fn new(provider: T) -> Self {
        Self { provider }
    }

    pub async fn scrape(&self, context: &Context) -> anyhow::Result<()> {
        self.provider.scrape(context).await
    }
}
