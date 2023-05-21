use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ActionMiddleware {
    async fn run(&self) -> Result<String>;
}