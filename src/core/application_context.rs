use surrealdb::{Surreal, engine::remote::ws::Client};

pub struct ApplicationContext {
    pub db: Surreal<Client>
}

impl ApplicationContext {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}