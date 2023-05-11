use anyhow::{Result, Ok};
use async_trait::async_trait;
use chrono::{format::format, Utc};
use log::debug;
use serenity::{utils::MessageBuilder, model::{user::User, prelude::{Emoji, UserId}}};

use crate::{core::{middlewares::ActionMiddleware, application_context::ApplicationContext}, models::babyfoot_match::{BabyfootPlayer, BabyfootPlayerInput, BabyfootMatchInput, BabyfootScore, BabyfootScoreInput, BabyfootMatch}};

pub struct BabyfootMiddleware<'a> {
    app: &'a ApplicationContext
}

impl<'a> BabyfootMiddleware<'a> {
    pub fn new(app: &'a ApplicationContext) -> Self {
        Self { app }
    }

    pub async fn last_ten(&self)  -> Result<String> {
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

    pub async fn register_match(&self, j1: &User, j2: &User, score_j1: i32, score_j2: i32) -> Result<String> {
        let db = &self.app.db;

        let mut p1: BabyfootPlayer;
        let mut p2: BabyfootPlayer;

        debug!("Checking J1 in database...");
        let mut response = db
            .query("SELECT * FROM babyfoot_player WHERE discord_id = '$discord_id'")
            .bind(("discord_id", j1.id.to_string()))
            .await?;

        let j1_opt: Option<BabyfootPlayer> = response.take(0)?;

        if j1_opt.is_none() {
            p1 = db
            .create("babyfoot_player")
            .content(BabyfootPlayerInput { 
                discord_id: j1.id.to_string(), 
                name: j1.name.clone(), 
                tag: j1.tag()
            })
            .await?;
        } else {
            p1 = j1_opt.unwrap();
        }

        debug!("Checking J2 in database...");
        let mut response = db
            .query("SELECT * FROM babyfoot_player WHERE discord_id = '$discord_id'")
            .bind(("discord_id", j2.id.to_string()))
            .await?;

        let j2_opt: Option<BabyfootPlayer> = response.take(0)?;

        if j2_opt.is_none() {
            p2 = db
            .create("babyfoot_player")
            .content(BabyfootPlayerInput { 
                discord_id: j2.id.to_string(), 
                name: j2.name.clone(), 
                tag: j2.tag()
            })
            .await?;
        } else {
            p2 = j2_opt.unwrap();
        }

        let c_match = BabyfootMatchInput {
            date: Utc::now(),
            player_1: BabyfootScoreInput {
                player: p1,
                score: score_j1,
            },
            player_2: BabyfootScoreInput {
                player: p2,
                score: score_j2,
            },
        };

        let _created: BabyfootMatch = db
            .create("babyfoot_match")
            .content(c_match)
            .await?;

        debug!("Creating register_match response...");
        let message = MessageBuilder::new()
            .quote_rest()
            .push_line("  ")
            .push_line(
                MessageBuilder::new()
                .push("⚽")
                .push("")
                .push("🥳")
                .push_bold("    Joli match !")
                .build()
            )
            .push_line("")
            .push_line(
                MessageBuilder::new()
                .mention(j1)
                .push(" ")
                .push(score_j1)
                .push("   -   ")
                .push(score_j2)
                .push(" ")
                .mention(j2)
                .build()
            )
            .push_line("")
            .push_line("Allez maintenant on retourne travailler")

            .build();

        Ok(message)
    }
}
