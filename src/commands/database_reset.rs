use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use serenity::Client;

use crate::core::{application_context::ApplicationContext, commands::AsyncCommand};

pub struct DatabaseResetCommand;

impl DatabaseResetCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AsyncCommand for DatabaseResetCommand {
    fn get_name(&self) -> String {
        "db:reset".into()
    }

    async fn run(
        &self,
        _discord: Client,
        app: ApplicationContext,
        _args: &HashMap<String, Option<String>>,
    ) -> Result<()> {
        debug!("Resetting database...");

        debug!("Removing babyfoot matches");
        app.db.query("delete babyfoot_match").await?;

        debug!("Removing persons");
        app.db.query("delete person").await?;

        debug!("Database reset !");

        Ok(())
    }
}
