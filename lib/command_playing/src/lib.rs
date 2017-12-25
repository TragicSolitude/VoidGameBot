#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;

use std::sync::Mutex;
use std::collections::HashMap;
use discord::Discord;
use discord::model::{Message, Channel};
use common_void::feature;

lazy_static! {
    static ref CHANNELS: Mutex<HashMap<String, Channel>> = Mutex::new(HashMap::new());
}

/// Plugin feature usage description; gets called at link-time and returns a
/// bitmask of all features used by this plugin
#[no_mangle]
pub extern fn describe() -> u32 {
    feature::TEST
}

/// Will create a new channel managed by the bot with the name given in the
/// first argument. This will just move the user into said channel if it already
/// exists and is managed. If it exists and isn't managed by the bot it will
/// throw an error to prevent manipulating the bot for private/restricted
/// channels.
///
/// Error codes:
/// `100` - Invalid argument count
///
/// # Arguments
/// Standard VGB plugin arguments
///
/// * `discord` - A reference to the discord instance
/// * `message` - A reference to the received message instance
/// * `args`    - A vector of all parameters specified for the command
#[no_mangle]
pub extern fn main(discord: &Discord, message: &Message, args: Vec<String>) -> i16 {
    if args.len() < 1 {
        return 100;
    }

    0
}