

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use super::person::Person;

#[derive(Debug, Serialize)]
pub struct BabyfootScoreInput {
    pub player: Person,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootScore {
    pub player: Person,
    pub score: i32,
}

#[derive(Debug, Serialize)]
pub struct BabyfootMatchInput {
    pub date: DateTime<Utc>,
    pub player_1: BabyfootScoreInput,
    pub player_2: BabyfootScoreInput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootMatch {
    pub id: Thing,
    pub date: DateTime<Utc>,
    pub player_1: BabyfootScore,
    pub player_2: BabyfootScore,
}

pub async fn create_match_from_players_and_scores(
    db: &Surreal<Client>,
    data: Vec<(Person, i32)>,
) -> Result<BabyfootMatch> {
    let p1_data = data.get(0).unwrap();
    let p2_data = data.get(1).unwrap();

    let input = BabyfootMatchInput {
        date: Utc::now(),
        player_1: BabyfootScoreInput {
            player: p1_data.0.clone(),
            score: p1_data.1,
        },
        player_2: BabyfootScoreInput {
            player: p2_data.0.clone(),
            score: p2_data.1,
        },
    };

    let created: BabyfootMatch = db.create("babyfoot_match").content(input).await?;

    Ok(created)
}
