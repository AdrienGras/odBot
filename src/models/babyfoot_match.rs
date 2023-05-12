use std::todo;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use super::{babyfoot_team::BabyfootTeam, person::Person};

#[derive(Debug, Serialize)]
pub struct BabyfootMatchInput {
    pub date: DateTime<Utc>,
    pub team_1: BabyfootTeam,
    pub team_2: BabyfootTeam,
    pub score_team_1: i32,
    pub score_team_2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootMatch {
    pub id: Thing,
    pub date: DateTime<Utc>,
    pub team_1: BabyfootTeam,
    pub team_2: BabyfootTeam,
    pub score_team_1: i32,
    pub score_team_2: i32,
}

pub async fn create_match_from_players_and_scores(
    db: &Surreal<Client>,
    data: Vec<(Vec<Person>, i32)>,
) -> Result<BabyfootMatch> {
    todo!();
}
