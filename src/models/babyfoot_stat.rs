use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use super::person::Person;

#[derive(Debug, Serialize)]
pub struct BabyfootStatInput {
    pub player: Person,
    pub score: i32,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootStat {
    pub id: Thing,
    pub player: Person,
    pub score: i32,
    pub date: DateTime<Utc>,
}

pub async fn create(db: Surreal<Client>, person: &Person, score: i32) -> Result<BabyfootStat> {
    let created: BabyfootStat = db
        .create("babyfoot_stat")
        .content(BabyfootStatInput {
            player: person.clone(),
            score,
            date: Utc::now(),
        })
        .await?;

    Ok(created)
}
