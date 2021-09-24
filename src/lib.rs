#[macro_use]
extern crate lazy_static;

use traits::CommandRegistrar;

pub mod log;
pub mod macros;
pub mod traits;
pub mod structs;
pub mod message;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

custom_error::custom_error! { pub CommandError
    ExecutionFailure { message: &'static str } = "Failed to execute command: {}",
    Other { message: &'static str } = "{}",
}

pub mod userservice {
    tonic::include_proto!("userservice");
}

pub mod youtubeservice {
    tonic::include_proto!("youtubeservice");
}

/// Information about the API that will be embedded into the library.
pub struct CommandDeclaration {
    /// The rustc version that was used to compile API
    pub rustc_version: &'static str,
    /// The API version
    pub core_version: &'static str,
    /// The register function for registering a new command
    pub register: unsafe extern "C" fn(&mut dyn CommandRegistrar),
}