use chrono::NaiveDateTime;
use log::error;
use super::userservice::user_service_client::UserServiceClient;
use super::youtubeservice::you_tube_service_client::YouTubeServiceClient;
use tonic::transport::Channel;

use crate::message::StringView;

fn from_prost_timestamp(prost_timestamp: &prost_types::Timestamp) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(prost_timestamp.seconds, prost_timestamp.nanos as u32)
}

pub struct ServiceDirectory<'a> {
    pub userservice_client: &'a UserServiceClient<Channel>,
    pub youtubeservice_client: &'a YouTubeServiceClient<Channel>,
}

/// A user that sent a message.
pub struct CommandUser {
    pub channel_id: String,
    pub display_name: String,
    pub active_time: i64,
    pub money: f64,
    pub first_seen_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

impl From<super::userservice::BppUser> for CommandUser {
    fn from(user: super::userservice::BppUser) -> Self {
        let first_seen = from_prost_timestamp(user.first_seen_at.as_ref().unwrap());
        let last_seen = from_prost_timestamp(user.last_seen_at.as_ref().unwrap());
        CommandUser {
            channel_id: user.channel_id,
            display_name: user.display_name,
            active_time: user.hours.unwrap().seconds,
            money: user.money,
            first_seen_at: first_seen,
            last_seen_at: last_seen,
        }
    }
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
        if let Ok(args) = args {
            let command_name = args.first().unwrap().clone();
            if args.len() > 1 {
                let command_args: Vec<String> = args.iter().skip(1).cloned().collect();
                Message {
                    user,
                    message: message_copy,
                    command_name,
                    command_args,
                    has_command_info: true,
                }
            } else {
                Message {
                    user,
                    message: message_copy,
                    command_name,
                    command_args: Vec::new(),
                    has_command_info: true,
                }
            }
        } else {
            error!("Unable to get parameters from message: {}", args.unwrap_err());
            Message {
                user,
                message: message_copy,
                has_command_info: false,
                command_name: String::new(),
                command_args: Vec::new(),
            }
        }
    }
}