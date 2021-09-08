use async_trait::async_trait;
use dyn_clone::DynClone;

use super::CommandError;
use crate::structs::Message;

/// Types that implement this trait can be registered as a command handler.
///
/// This trait is an async_trait, which means that you can use async/await syntax.
#[async_trait]
pub trait Command<YT>: Send + Sync + DynClone where YT: YouTubeSendable + ?Sized {
    async fn execute(&self, message: Message, sendable: &YT) -> Result<(), CommandError>;
}
dyn_clone::clone_trait_object!(Command<dyn YouTubeSendable>);

/// Types that implement this trait register commands.
pub trait CommandRegistrar {
    fn register_command(&mut self, name: &str, aliases: &[&str], command: Box<dyn Command<dyn YouTubeSendable>>);
    fn send_message(&self, message: &str);
}

#[async_trait]
pub trait YouTubeSendable: Send + Sync {
    async fn send_message(&self, message: &str);
}