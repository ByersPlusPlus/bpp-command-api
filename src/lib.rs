#[macro_use]
extern crate lazy_static;

use async_trait::async_trait;
use dyn_clone::DynClone;
use structs::Message;

pub mod log;
pub mod macros;
pub mod structs;
pub mod message;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

custom_error::custom_error! { pub CommandError
    ExecutionFailure { message: &'static str } = "Failed to execute command: {}",
    Other { message: &'static str } = "{}",
}

/// Types that implement this trait can be registered as a command handler.
///
/// This trait is an async_trait, which means that you can use async/await syntax.
#[async_trait]
pub trait Command: Send + Sync + DynClone {
    async fn execute(&self, message: Message) -> Result<(), CommandError>;
}

dyn_clone::clone_trait_object!(Command);

/// Information about the API that will be embedded into the library.
pub struct CommandDeclaration {
    /// The rustc version that was used to compile API
    pub rustc_version: &'static str,
    /// The API version
    pub core_version: &'static str,
    /// The register function for registering a new command
    pub register: unsafe extern "C" fn(&mut dyn CommandRegistrar),
}

/// Types that implement this trait register commands.
pub trait CommandRegistrar {
    fn register_command(&mut self, name: &str, aliases: &[&str], command: Box<dyn Command>);
}

/// Exports a command for it to be loaded.
///
/// # Example
///
/// ```
/// use async_trait::async_trait;
/// use bpp_command_api::{Command, CommandRegistrar};
///
/// #[derive(Clone)]
/// pub struct AddCanCommand;
///
/// #[async_trait]
/// impl Command for AddCanCommand {
///     async fn execute(&self, message: bpp_command_api::Message) {
///         println!("Added a can!");
///     }
/// }
///
/// bpp_command_api::export_command!(register);
///
/// extern "C" fn register(registrar: &mut dyn CommandRegistrar) {
///     registrar.register_command("addcan", &["addbear", "addjohn"], Box::new(AddCanCommand));
/// }
/// ```
#[macro_export]
macro_rules! export_command {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static command_declaration: $crate::CommandDeclaration = $crate::CommandDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}