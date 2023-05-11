use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use log::Record;
use serenity::Client;

use crate::{core::{application_context::ApplicationContext, commands::AsyncCommand}, models::babyfoot_match::{BabyfootPlayer, BabyfootPlayerInput}};

pub struct TestBDCommand;

impl TestBDCommand {
    pub fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl AsyncCommand for TestBDCommand {
    fn get_name(&self) -> String {
        "test:db".into()
    }

    async fn run(&self, _discord: Client, app: ApplicationContext, _args: &HashMap<String, Option<String>>) -> Result<()> {
        app.db.query("delete babyfoot_player").await?;

        let created: BabyfootPlayer = app.db
            .create("babyfoot_player")
            .content(BabyfootPlayerInput { 
                discord_id: "abc".into(), 
                name: "arno".into(), 
                tag: "pd".into()
            })
            .await?;

        dbg!(created);

        let people: Vec<BabyfootPlayer> = app.db.select("babyfoot_player").await?;
        dbg!(people);

        let mut result = app.db.query("SELECT * FROM babyfoot_player WHERE name = 'arno'").await?;
        let pd: Option<BabyfootPlayer> = result.take(0)?;
        dbg!(pd);

        Ok(())
    }
}