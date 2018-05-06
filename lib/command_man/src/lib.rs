#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;
extern crate regex;

use std::sync::Mutex;
use std::collections::HashMap;
use discord::Discord;
use discord::model::Message;
use common_void::{feature, hook};
use common_void::plugin::Plugin;
use regex::Regex;

lazy_static! {
    static ref MANUALS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref RE: Regex = Regex::new(r"^libcommand_(?P<name>.*)$").unwrap();
}

#[no_mangle]
pub extern fn describe() -> u64 {
    feature::TEST | hook::PLUGIN_LOAD
}

/// Used to display documentation for currently loaded plugins
#[no_mangle]
pub extern fn main(discord: &Discord, message: &Message, args: Vec<String>) -> u16 {
    if args.len() < 1 {
        return 100;
    }

    let lock = match MANUALS.lock() {
        Ok(manuals) => manuals,
        Err(_) => return 200
    };

    match lock.get(&args[0]) {
        Some(content) => {
            match discord.send_message(message.channel_id, &format!("```\n{}\n```", content), "", false) {
                Ok(_) => {
                    return 0
                },
                Err(_) => {
                    return 0
                }
            }
        },
        None => {
            return 200;
        }
    }
}

/// Uses the plugin load hook for loading documentation from plugins using a
/// call to a `manual` function
#[no_mangle]
pub extern fn hook_plugin_load(plugin: &Plugin) -> u16 {
    let mut lock = MANUALS.lock().unwrap();
    match unsafe { plugin.link.get::<extern fn() -> String>(b"manual") } {
        Ok(function) => {
            let fullname = &plugin.name;
            let caps = RE.captures(fullname).unwrap();
            let name = caps["name"].to_string();

            lock.insert(name.clone(), function());
            println!("[HKK; PLL]    man; add key {}", name);
        },
        Err(_) => return 0
    };

    0
}