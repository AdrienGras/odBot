use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use serenity::Client;

use crate::commands::test_db::TestBDCommand;

use super::application_context::ApplicationContext;

#[async_trait]
pub trait AsyncCommand: Send + Sync {
    fn get_name(&self) -> String;
    async fn run(&self, discord: Client, app: ApplicationContext, args: &HashMap<String, Option<String>>) -> Result<()>;
}

pub struct ConsoleCommandRegistry {
    /// the actual registry of AsyncCommand
    commands: HashMap<String, Arc<dyn AsyncCommand>>,
}

impl ConsoleCommandRegistry {
    /// creates a new instance.
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, Arc<dyn AsyncCommand>>::new(),
        }
    }

    /// adds a ConsoleCommand to the registry, using it's name as a key.
    ///
    /// **Note:** The command instance must be Arc'ed.
    pub fn add(&mut self, command: Arc<dyn AsyncCommand>) -> &mut Self {
        self.commands.insert(command.get_name(), command);

        self
    }

    /// gets a given command by it's unique name.
    pub fn get(&self, command_name: &str) -> Option<&Arc<dyn AsyncCommand>> {
        self.commands.get(command_name)
    }

    /// gets names of all the commands present in the current registry.
    pub fn get_all_names(&self) -> Vec<String> {
        return self.commands.keys().cloned().collect();
    }
}

pub fn get_command_registry() -> ConsoleCommandRegistry {
    let mut registry = ConsoleCommandRegistry::new();

    // insert all wanted command here
    registry.add(Arc::new(TestBDCommand::new()));

    registry
}