extern crate libloading;
extern crate regex;
extern crate discord;
extern crate common_void;

mod plugin;

use libloading::{Library, Symbol};
use std::{env, fs};
use std::collections::HashMap;
use discord::Discord;
use discord::model::{Event, Message, ServerId, VoiceState};
use plugin::Plugin;
use common_void::{hook, filter};

fn main() {
    let plugins = fs::read_dir("plugins/").unwrap();
    let mut libs: HashMap<String, Plugin> = HashMap::new();

    for plugin in plugins {
        let plugin = plugin.unwrap();
        let mut key = plugin.file_name().into_string().unwrap();

        // Cut off file extension
        // TODO Platform specific stuff
        let len = key.len();
        key.truncate(len - 3);

        let lib = Library::new(plugin.path()).unwrap();

        libs.insert(key, Plugin::new(lib));
    }
    
    let discord = Discord::from_bot_token(
        &env::var("VOIDGAMEBOT_TOKEN").expect("No token found in env:VOIDGAMEBOT_TOKEN.")
    ).expect("Login with given bot token failed.");
    // get_current_user() does not work due to a bug in discord-rs crate
    // let bot_id = discord.get_current_user().unwrap().id;

    // TODO setup interval feature
    // Like a hook but just one that gets called on an interval with no params
    // Useful for implementing something like a periodic buffer flush to file

    let (mut connection, _) = discord.connect().expect("Failed to connect.");

    println!("Connected.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                match message.content.chars().nth(0) {
                    Some(character) => {
                        if character != '!' {
                            continue
                        }
                    }
                    None => {
                        continue;
                    }
                }

                println!("[REC]         {}", message.content);

                let mut opts = message.content.split_whitespace();
                let mut command = opts.next().unwrap();
                command = &command[1..command.len()];
                
                let mut command = command.to_string();
                for lib in libs.values() {
                    if lib.description & filter::BEFORE_PLUGIN_LOOKUP != 0 {
                        let filter: std::io::Result<Symbol<extern fn(String) -> String>> = unsafe {
                            lib.link.get(b"filter_before_plugin_lookup")
                        };

                        match filter {
                            Ok(function) => command = function(command),
                            Err(err) => println!("[CMD; ERR]    dll--{:?}", err)
                        };
                    }
                }
                let command = &command;

                // filter_before_plugin_lookup
                let key = format!("{}{}", "libcommand_", command);

                println!("[REQ; KEY]    {}; {}", command, key);

                if libs.contains_key(&key) {
                    let mut args: Vec<String> = Vec::new();

                    for opt in opts {
                        args.push(opt.to_string());
                    }

                    println!("[EXE; ARG]    {}; [{}]", command, args.join(", "));

                    let lib = libs.get(&key).unwrap();
                    let func: Symbol<extern fn(&Discord, &Message, Vec<String>) -> u16> = unsafe {
                        lib.link.get(b"main").unwrap()
                    };

                    // TODO Try out futures/threading to improve performance for
                    // potentially long-running commands
                    let res = func(&discord, &message, args);

                    if res != 0 {
                        let msg = format!("[ERR]         Command exited with non-zero code {}", res);
                        println!("{}", msg);
                        match discord.send_message(message.channel_id, &format!("`{}`", msg), "", false) {
                            Ok(_) => {},
                            Err(err) => {
                                println!("[ERR]!        Could not notify channel; {:?}", err)
                            }
                        };
                    } else {
                        println!("[SUC]");
                    }
                } else {
                    // TODO Check for lib and load it here
                    println!("[MIS]");
                }
                
            }
            Ok(Event::VoiceStateUpdate(server_id, state)) => {
                // TODO make hook system more re-usable for other types of events
                for lib in libs.values() {
                    if lib.description & hook::VOICE_STATE_UPDATE != 0 {
                        let hook: std::io::Result<Symbol<extern fn(&Discord, &ServerId, &VoiceState) -> u16>> = unsafe {
                            lib.link.get(b"hook_voice_state_update")
                        };

                        match hook {
                            Ok(function) => {
                                let res = function(&discord, &server_id.unwrap(), &state);

                                if res > 0 {
                                    println!("[VSU; ERR]    {}", res);
                                } else {
                                    println!("[VSU; SUC]");
                                }
                            }
                            Err(err) => {
                                println!("[VSU; ERR]    dll--{:?}", err);
                            }
                        };
                    }
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