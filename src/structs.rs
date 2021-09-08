use chrono::NaiveDateTime;
use log::error;

use crate::message::StringView;

/// A user that sent a message.
pub struct CommandUser {
    pub channel_id: String,
    pub display_name: String,
    pub active_time: i64,
    pub money: f64,
    pub first_seen_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

/// A message sent by a user.
///
/// May contain command information.
pub struct Message {
    /// The user that sent this message
    pub user: CommandUser,
    /// The raw message
    pub message: String,
    /// Flag indicating if this message can be used for command parsing
    pub has_command_info: bool,
    /// The command that was sent
    ///
    /// This is only set if `has_command_info` is true and is usually the first word of a message.
    pub command_name: String,
    /// The arguments that were sent with the command
    ///
    /// If `has_command_info` is false, this will be empty. This can also be empty if there were simply no arguments.
    pub command_args: Vec<String>,
}

impl CommandUser {
    /// Creates a new CommandUser
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

    /// Creates a mock user to use with testing
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
    /// Creates a new Message from a user and a raw String
    ///
    /// If a StringView cannot determine the command structure, it will set `has_command_info` to false.
    ///
    /// If the message is a command, it will set `has_command_info` to true and set the command name and arguments.
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