use anyhow::Result;
use log::debug;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::data::static_data;

#[derive(Debug, Serialize)]
pub struct BabyfootMatchQuoteInput {
    pub quote: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootMatchQuote {
    pub id: Thing,
    pub quote: String,
}

pub async fn load(db: &Surreal<Client>) -> Result<()> {
    debug!("Collecting data...");
    let data = static_data::babyfoot_quotes();

    debug!("Removing old data...");

    let _: Vec<BabyfootMatchQuote> = db.delete("babyfoot_quote").await?;

    for quote in data.iter() {
        let input = BabyfootMatchQuoteInput {
            quote: quote.to_string(),
        };

        debug!("Inserting: {}", quote);
        let _: BabyfootMatchQuote = db.create("babyfoot_quote").content(input).await?;
    }

    Ok(())
}

pub async fn random(db: &Surreal<Client>) -> Result<String> {
    let result: Vec<BabyfootMatchQuote> = db.select("babyfoot_quote").await?;

    if result.is_empty() {
        return Ok("Bien joué !".into());
    }

    let selected = result.choose(&mut thread_rng()).unwrap();

    Ok(selected.quote.clone())
}
