use std::vec;

use anyhow::{Ok, Result};

use log::debug;
use serenity::{
    model::{prelude::UserId, user::User},
    utils::MessageBuilder,
};

use crate::{
    core::application_context::ApplicationContext,
    models::{
        babyfoot_match::{self, BabyfootMatch},
        babyfoot_match_quote,
        person::{self, Person},
    },
};

pub struct BabyfootMiddleware<'a> {
    app: &'a ApplicationContext,
}

impl<'a> BabyfootMiddleware<'a> {
    pub fn new(app: &'a ApplicationContext) -> Self {
        Self { app }
    }

    pub async fn last_ten(&self) -> Result<String> {
        let db = &self.app.db;

        let mut response = db
            .query("SELECT * FROM babyfoot_match ORDER BY date DESC LIMIT 10")
            .await?;

        let possible_matches: Vec<Option<BabyfootMatch>> = response.take(0)?;

        debug!("Creating register_match response...");
        let title = MessageBuilder::new()
            .push_bold("Les 10 derniers matchs")
            .build();

        let mut mb = &mut MessageBuilder::new();
        mb = mb.quote_rest();
        mb = mb.push_line("  ");
        mb = mb.push_line(title);

        for m in possible_matches.iter() {
            let m = m.as_ref().unwrap();

            let m_line = MessageBuilder::new()
                .mention(&UserId(m.player_1.player.discord_id.parse::<u64>()?))
                .push(" ")
                .push(m.player_1.score)
                .push("   -   ")
                .push(m.player_2.score)
                .push(" ")
                .mention(&UserId(m.player_2.player.discord_id.parse::<u64>()?))
                .build();

            mb = mb.push_line(m_line);
        }

        let message = mb.build();

        Ok(message)
    }

    pub async fn register_match(
        &self,
        j1: &User,
        j2: &User,
        score_j1: i32,
        score_j2: i32,
    ) -> Result<String> {
        let db = &self.app.db;

        debug!("Checking J1 in database...");
        let p1: Person = person::get_or_create_person_from_discord_user(db, j1).await?;

        debug!("Checking J2 in database...");
        let p2: Person = person::get_or_create_person_from_discord_user(db, j2).await?;

        let _created = babyfoot_match::create_match_from_players_and_scores(
            db,
            vec![(p1, score_j1), (p2, score_j2)],
        )
        .await?;

        debug!("Creating register_match response...");
        let message = MessageBuilder::new()
            .quote_rest()
            .push_line("  ")
            .push_line(
                MessageBuilder::new()
                    .push("⚽ 🥳")
                    .push_bold(format!(
                        "       {}",
                        babyfoot_match_quote::random(db).await?
                    ))
                    .build(),
            )
            .push_line("")
            .push_line(
                MessageBuilder::new()
                    .mention(j1)
                    .push(" ")
                    .push(score_j1)
                    .push(" — ")
                    .push(score_j2)
                    .push(" ")
                    .mention(j2)
                    .build(),
            )
            .push_line("")
            .push_line("Allez maintenant on retourne travailler")
            .push_line("_ _")
            .build();

        Ok(message)
    }
}
