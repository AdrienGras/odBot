use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use serenity::Client;

use crate::{
    core::{application_context::ApplicationContext, commands::AsyncCommand},
    models::babyfoot_match_quote,
};

pub struct ImportBabyfootQuotesCommand;

impl ImportBabyfootQuotesCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AsyncCommand for ImportBabyfootQuotesCommand {
    fn get_name(&self) -> String {
        "import:babyfoot:quotes".into()
    }

    async fn run(
        &self,
        _discord: Client,
        app: ApplicationContext,
        _args: &HashMap<String, Option<String>>,
    ) -> Result<()> {
        debug!("Importing babyfoot quotes...");
        babyfoot_match_quote::load(&app.db).await?;

        debug!("Selecting one random...");
        let quote = babyfoot_match_quote::random(&app.db).await?;

        debug!("Quote: {}", quote);

        debug!("Quote imported !");
        Ok(())
    }
}
