use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use serenity::{utils::MessageBuilder, model::user::User};

use crate::core::middlewares::ActionMiddleware;

pub struct PingMiddleware<'a> {
    user: &'a User
}

impl<'a> PingMiddleware<'a> {
    pub fn new(user: &'a User) -> Self {
        Self { user }
    }
}

#[async_trait]
impl<'a> ActionMiddleware for PingMiddleware<'a> {
    async fn run(&self) -> Result<String> {
        debug!("Creating ping response...");
        let message = MessageBuilder::new()
            .push_quote("Hello ")
            .mention(self.user)
            .push(" it ")
            .push_bold("works !")
            .build();

        Ok(message)
    }
}