#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;

use std::sync::Mutex;
use std::collections::HashMap;
use discord::Discord;
use discord::model::Message;
use common_void::{feature, filter};

lazy_static! {
    // TODO Add file backing
    static ref ALIASES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern fn describe() -> u64 {
    feature::TEST | filter::BEFORE_PLUGIN_LOOKUP
}

/// Lets you alias commands
///
/// Usage:
/// !alias pl playing
///
/// This will allow pl to work as if it was its own command but it will instead
/// just be transformed into the playing command just before plugin lookup
///
/// Error codes:
/// `100` - Invalid argument count
/// `200` - Error locking aliases map
///
/// # Arguments
/// Standard VGB plugin arguments
#[no_mangle]
pub extern fn main(_discord: &Discord, _message: &Message, args: Vec<String>) -> u16 {
    if args.len() < 2 {
        return 100;
    }

    let mut lock = match ALIASES.lock() {
        Ok(aliases) => aliases,
        Err(_) => return 200
    };

    println!("[CMD; ALI]    Alias {} -> {}", args[0], args[1]);

    lock.insert(args[0].clone(), args[1].clone());
    0
}

#[no_mangle]
pub extern fn filter_before_plugin_lookup(command: String) -> String {
    let mut lock = match ALIASES.lock() {
        Ok(aliases) => aliases,
        Err(_) => return command
    };
    
    match lock.get(&command) {
        Some(alias) => {
            println!("[CMD; ALI]    Resolve alias {} to {}", command, alias);
            alias.to_string()
        }
        None => command
    }
}