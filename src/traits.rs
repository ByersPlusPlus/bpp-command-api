use async_trait::async_trait;
use dyn_clone::DynClone;

use super::CommandError;
use crate::structs::{Message, ServiceDirectory};

/// Types that implement this trait can be registered as a command handler.
///
/// This trait is an async_trait, which means that you can use async/await syntax.
#[async_trait]
pub trait Command: Send + Sync + DynClone {
    async fn execute(&self, message: Message, service_directory: &mut ServiceDirectory) -> Result<(), CommandError>;
}
dyn_clone::clone_trait_object!(Command);

/// Types that implement this trait register commands.
pub trait CommandRegistrar {
    fn register_command(&mut self, name: &str, aliases: &[&str], command: Box<dyn Command>);
}