use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::person::Person;

#[derive(Debug, Serialize)]
pub struct BabyfootTeamInput {
    pub players: Vec<Person>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BabyfootTeam {
    pub players: Vec<Person>,
}
