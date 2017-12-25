extern crate libloading;
extern crate regex;
extern crate discord;

mod platform;

use libloading::{Library, Symbol};
use std::{env, fs};
use std::collections::HashMap;
use discord::Discord;
use discord::model::{Event, Message, UserId};

fn main() {
    let plugins = fs::read_dir("plugins/").unwrap();
    let mut libs: HashMap<String, Library> = HashMap::new();

    for plugin in plugins {
        let plugin = plugin.unwrap();
        let mut key = plugin.file_name().into_string().unwrap();

        // Cut off file extension
        // TODO Platform specific stuff
        let len = key.len();
        key.truncate(len - 3);

        let lib = Library::new(plugin.path()).unwrap();
        // TODO Use describe function for getting feature usage
        libs.insert(key, lib);
    }
    
    let discord = Discord::from_bot_token(
        &env::var("VOIDGAMEBOT_TOKEN").expect("No token found in env:VOIDGAMEBOT_TOKEN.")
    ).expect("Login with given bot token failed.");
    // get_current_user() does not work due to a bug in discord-rs crate
    // let bot_id = discord.get_current_user().unwrap().id;

    let (mut connection, _) = discord.connect().expect("Failed to connect.");
    println!("Connected.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                if message.content.chars().nth(0).unwrap() != '!' {
                    continue;
                }

                println!("[REC]         {}", message.content);

                let mut opts = message.content.split_whitespace();
                let command = opts.next().unwrap();
                let command = &command[1..command.len()];
                let key = format!("{}{}", "libcommand_", command);

                println!("[REQ; KEY]    {}; {}", command, key);

                if libs.contains_key(&key) {
                    let mut args: Vec<String> = Vec::new();

                    for opt in opts {
                        args.push(opt.to_string());
                    }

                    println!("[EXE; ARG]    {}; [{}]", command, args.join(", "));

                    let lib = libs.get(&key).unwrap();
                    let func: Symbol<extern fn(&Discord, &Message, Vec<String>) -> u8> = unsafe {
                        lib.get(b"main").unwrap()
                    };
                    // TODO Try out futures/threading to improve performance for
                    // potentially long-running commands
                    let res = func(&discord, &message, args);

                    if res != 0 {
                        let msg = format!("[ERR]         Command exited with non-zero code {}", res);
                        println!("{}", msg);
                        match discord.send_message(message.channel_id, &format!("`{}`", msg), "", false) {
                            Ok(_) => {},
                            Err(_) => println!("[ERR]!        Could not notify channel")
                        };
                    } else {
                        println!("[SUC]");
                    }
                } else {
                    // TODO Check for lib and load it here
                    println!("[MIS]");
                }
                
            }
            // TODO Create hooks that libs can attach to (call some kind of
            // descriptor function when loading the lib?) for different events
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Error: {:?}", err)
        }
    }
}