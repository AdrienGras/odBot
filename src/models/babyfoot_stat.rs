use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use super::person::Person;

#[derive(Debug, Serialize)]
pub struct BabyfootStatInput {
    pub player_id: String,
    pub score: i32,
    pub is_winning: bool,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootStat {
    pub id: Thing,
    pub player_id: String,
    pub score: i32,
    pub is_winning: bool,
    pub date: DateTime<Utc>,
}

pub async fn create(db: &Surreal<Client>, person: &Person, score: i32, is_winning: bool) -> Result<BabyfootStat> {
    let created: BabyfootStat = db
        .create("babyfoot_stat")
        .content(BabyfootStatInput {
            player_id: person.clone().discord_id,
            score,
            is_winning,
            date: Utc::now(),
        })
        .await?;

    Ok(created)
}

pub async fn find_all_by_person(db: &Surreal<Client>, person: &Person) -> Result<Vec<BabyfootStat>> {
    let result: Vec<BabyfootStat> = db
        .query("SELECT * FROM babyfoot_stat WHERE player_id = $p_id")
        .bind(("p_id", &person.discord_id))
        .await?
        .take(0)?;

    Ok(result)
}

pub async fn all(db: &Surreal<Client>) -> Result<Vec<BabyfootStat>> {
    let result: Vec<BabyfootStat> = db
        .select("babyfoot_stat")
        .await?;

    Ok(result)
}
