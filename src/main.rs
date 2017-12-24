extern crate libloading;
extern crate regex;
extern crate discord;

mod platform;

use libloading::{Library, Symbol};
use std::{env, fs};
use std::collections::HashMap;
use platform::Platform;
use discord::Discord;
use discord::model::Event;

fn main() {
    let plugins = fs::read_dir("plugins/").unwrap();
    let mut libs: HashMap<String, Library> = HashMap::new();

    // TODO Platform detection for lib file extensions
    let platform = Platform::NIX;

    for plugin in plugins {
        let plugin = plugin.unwrap();
        let mut key = plugin.file_name().into_string().unwrap();

        // Cut off file extension
        let len = key.len();
        key.truncate(len - 3);

        let lib = Library::new(plugin.path()).unwrap();
        libs.insert(key, lib);
    }
    
    let discord = Discord::from_bot_token(
        &env::var("VOIDGAMEBOT_TOKEN").expect("No token found in env:VOIDGAMEBOT_TOKEN.")
    ).expect("Login with given bot token failed.");

    let (mut connection, _) = discord.connect().expect("Failed to connect.");
    println!("Connected.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("Received message: {}", message.content);

                let mut opts = message.content.split_whitespace();
                let command = opts.next().unwrap();
                let key = format!("{}{}", "libcommand_", command);

                println!("Command extracted: {}; Checking for key {}...", command, key);

                if libs.contains_key(&key) {
                    let mut args: Vec<String> = Vec::new();

                    for opt in opts {
                        args.push(opt.to_string());
                    }

                    let lib = libs.get(&key).unwrap();
                    let func: Symbol<extern fn(Vec<String>) -> u8> = unsafe {
                        lib.get(b"main").unwrap()
                    };
                    func(args);
                } else {
                    // TODO Check for lib and load it here
                }
                
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Error: {:?}", err)
        }
    }
}