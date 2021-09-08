#[macro_use]
extern crate lazy_static;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use dyn_clone::DynClone;
use ::log::error;
use message::StringView;

pub mod log;
pub mod macros;
pub mod message;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub struct CommandUser {
    pub channel_id: String,
    pub display_name: String,
    pub active_time: i64,
    pub money: f64,
    pub first_seen_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

pub struct Message {
    pub user: CommandUser,
    pub message: String,
    pub has_command_info: bool,
    pub command_name: String,
    pub command_args: Vec<String>,
}

impl CommandUser {
    pub fn new(channel_id: String, display_name: String, active_time: i64, money: f64, first_seen_at: NaiveDateTime, last_seen_at: NaiveDateTime) -> CommandUser {
        CommandUser {
            channel_id,
            display_name,
            active_time,
            money,
            first_seen_at,
            last_seen_at,
        }
    }

    pub fn mock() -> CommandUser {
        CommandUser {
            channel_id: "mock".to_string(),
            display_name: "mock".to_string(),
            active_time: 0,
            money: 0.0,
            first_seen_at: NaiveDateTime::from_timestamp(0, 0),
            last_seen_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

impl Message {
    pub fn new(user: CommandUser, message: String) -> Message {
        let message_copy = message.clone();
        let mut message_view = StringView::new(message);
        let args = message_view.get_parameters();
        if args.is_err() {
            error!("Unable to get parameters from message: {}", args.unwrap_err());
            return Message {
                user,
                message: message_copy,
                has_command_info: false,
                command_name: String::new(),
                command_args: Vec::new(),
            };
        } else {
            let args = args.unwrap();
            let command_name = args.first().unwrap().clone();
            if args.len() > 1 {
                let command_args: Vec<String> = args.iter().skip(1).map(|x| x.clone()).collect();
                return Message {
                    user,
                    message: message_copy,
                    command_name,
                    command_args,
                    has_command_info: true,
                };
            } else {
                return Message {
                    user,
                    message: message_copy,
                    command_name,
                    command_args: Vec::new(),
                    has_command_info: true,
                };
            }
        };
    }
}

// define a trait for loadable commands
#[async_trait]
pub trait Command: Send + Sync + DynClone {
    async fn execute(&self, message: Message);
}

dyn_clone::clone_trait_object!(Command);

pub struct CommandDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn CommandRegistrar),
}

pub trait CommandRegistrar {
    fn register_command(&mut self, name: &str, aliases: &[&str], command: Box<dyn Command>);
}

/// Exports your command for it to be loaded.
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