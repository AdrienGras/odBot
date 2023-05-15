use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use serenity::model::user::User;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

#[derive(Debug, Serialize)]
pub struct PersonInput {
    pub discord_id: String,
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub id: Thing,
    pub discord_id: String,
    pub name: String,
    pub tag: String,
}

pub async fn get_or_create_person_from_discord_user(
    db: &Surreal<Client>,
    user: &User,
) -> Result<Person> {
    let p: Person;

    let person_id = user.id.to_string();

    debug!("Searching for person with ID {:}", person_id);

    let mut response = db
        .query("SELECT * FROM person WHERE discord_id = $d_id")
        .bind(("d_id", person_id))
        .await?;

    let person_opt: Option<Person> = response.take(0)?;

    if person_opt.is_none() {
        debug!("Person not found, creating it...");
        p = db
            .create("person")
            .content(PersonInput {
                discord_id: user.id.to_string(),
                name: user.name.clone(),
                tag: user.tag(),
            })
            .await?;
    } else {
        p = person_opt.unwrap();
        debug!("Person found: {}", p.id.id);
    }

    Ok(p)
}
