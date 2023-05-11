use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
pub struct BabyfootPlayerInput {
    pub discord_id: String,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootPlayer {
    pub id: Thing,
    pub discord_id: String,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Serialize)]
pub struct BabyfootScoreInput {
    pub player: BabyfootPlayer,
    pub score: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootScore {
    pub player: BabyfootPlayer,
    pub score: i32
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