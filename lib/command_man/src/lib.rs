#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;

use std::sync::Mutex;
use std::collections::HashMap;
use discord::Discord;
use discord::model::Message;
use common_void::{feature, hook};
use common_void::plugin::Plugin;

lazy_static! {
    static ref MANUALS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern fn describe() -> u64 {
    feature::TEST | hook::PLUGIN_LOAD
}

/// Used to display documentation for currently loaded plugins
#[no_mangle]
pub extern fn main(discord: &Discord, _message: &Message, args: Vec<String>) -> u16 {
    if args.len() < 1 {
        return 100;
    }

    let mut lock = match MANUALS.lock() {
        Ok(manuals) => manuals,
        Err(_) => return 200
    };

    println!("[CMD; MAN]    Manual for command \"{}\"", args[0]);

    0
}

/// Uses the plugin load hook for loading documentation from plugins using a
/// call to a `manual` function
#[no_mangle]
pub extern fn hook_plugin_load(plugin: &Plugin) -> u16 {
    // Attempt to get manual function and run it
    0
}