/// Allows for inline definition of HashMaps and HashSets.
///
/// # Examples
///
/// ## HashMap
/// ```
/// let my_hashmap: HashMap<&str, &str> = collection! {
///    "key" => "value",
///    "key2" => "value2",
/// };
/// ```
///
/// ## HashSet
/// ```
/// let my_hashset: HashSet<&str> = collection! {
///    "value",
///    "value2",
/// };
/// ```
#[macro_export]
macro_rules! collection {
    ($($k:expr => $v:expr), * $(,)?) => {
        {
            use std::iter::{Iterator, IntoIterator};
            Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
        }
    };
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
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