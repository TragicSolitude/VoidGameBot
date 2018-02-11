#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;

mod managed_channel;

use std::sync::Mutex;
use std::collections::HashMap;
use discord::Discord;
use discord::model::{Message, ServerId, VoiceState, ChannelId, UserId, Channel, ChannelType};
use common_void::{feature, hook};
use managed_channel::ManagedChannel;

lazy_static! {
    // TODO Setup periodic flush to a file in case of sudden shutdown
    static ref CHANNELS: Mutex<HashMap<ChannelId, ManagedChannel>> = Mutex::new(HashMap::new());
    static ref CHANNEL_CURSTATE: Mutex<HashMap<UserId, ChannelId>> = Mutex::new(HashMap::new());
}

/// Plugin feature usage description; gets called at link-time and returns a
/// bitmask of all features used by this plugin
#[no_mangle]
pub extern fn describe() -> u64 {
    feature::TEST | hook::VOICE_STATE_UPDATE
}

fn get_user_server_id(discord: &Discord, user_id: &UserId) -> Result<ServerId, u16> {
    let state = match CHANNEL_CURSTATE.lock() {
        Ok(state) => state,
        Err(_) => return Err(201)
    };
    
    // Figure out what server the user is in
    match state.get(user_id) {
        Some(channel_id) => {
            match discord.get_channel(*channel_id) {
                Ok(Channel::Public(channel)) => {
                    return Ok(channel.server_id)
                },
                Ok(_) => {
                    println!("[ERR; PLY]   User is not in a public channel");
                    return Err(402);
                }
                Err(err) => {
                    println!("[ERR; PLY]   Error getting channel info; {:?}", err);
                    return Err(401);
                }
            }
        }
        None => return Err(300)
    }
}

/// Creates a new managed channel and returns the new channel id if successful
/// or it returns the existing channel id if a managed channel already exists
/// with the same name. In the case of failure this function returns None
fn create_managed_channel(discord: &Discord, user_id: &UserId, name: String) -> Result<ChannelId, u16> {
    let mut lock = match CHANNELS.lock() {
        Ok(channels) => channels,
        Err(_) => return Err(200)
    };

    let mut channel_id: Result<ChannelId, u16> = Err(404);
    for channel in lock.values() {
        if channel.name == name {
            channel_id = Ok(channel.id);
            break;
        }
    }

    match channel_id {
        Ok(_) => {}
        Err(_) => {
            println!("[CMD; PLY]    {}", "New Channel");

            let server_id = match get_user_server_id(discord, user_id) {
                Ok(id) => id,
                Err(_) => {
                    println!("[CMD; PLY]    Could not get user's current server id");
                    return Err(450)
                }
            };

            match discord.create_channel(server_id, &name, ChannelType::Voice) {
                Ok(Channel::Public(channel)) => {
                    lock.insert(channel.id, ManagedChannel::new(channel.id, name));
                    channel_id = Ok(channel.id);
                },
                Ok(_) => {
                    println!("[ERR; PLY]    Incorrect channel type created");
                    return Err(403);
                }
                Err(err) => {
                    println!("[ERR; PLY]    Could not create channel; {:?}", err);
                    return Err(400);
                }
            }
        }
    }

    channel_id
}

/// Will create a new channel managed by the bot with the name given in the
/// first argument. This will just move the user into said channel if it already
/// exists and is managed. If it exists and isn't managed by the bot it will
/// throw an error to prevent manipulating the bot for private/restricted
/// channels.
///
/// Error codes:
/// `100` - Invalid argument count
/// `200` - Error locking channels map
/// `201` - Error locking prev_channels map
/// `300` - User state unknown
/// `400` - Could not create channel
/// `401` - Could not get channel info
/// `402` - User channel not accessible
/// `403` - Wrong channel type created
/// `404` - Channel not found -- wow how funny am I a 404 joke /s
/// `500` - Could not move user into channel
///
/// # Arguments
/// Standard VGB plugin arguments
///
/// * `discord` - A reference to the discord instance
/// * `message` - A reference to the received message instance
/// * `args`    - A vector of all parameters specified for the command
#[no_mangle]
pub extern fn main(discord: &Discord, message: &Message, args: Vec<String>) -> u16 {
    if args.len() < 1 {
        return 100;
    }

    // let mut lock = match CHANNELS.lock() {
    //     Ok(channels) => channels,
    //     Err(_) => return 200
    // };
    // let state = match CHANNEL_CURSTATE.lock() {
    //     Ok(state) => state,
    //     Err(_) => return 201
    // };

    println!("[CMD; PLY]    Message from User ID: {}", message.author.id);

    let server_id = match get_user_server_id(discord, &message.author.id) {
        Ok(id) => id,
        Err(code) => return code
    };

    // Check if they are in a managed channel already
    // let mut is_managed = false;
    // for channel in lock.values() {
    //     if channel.name == args[0] {
    //         is_managed = true;
    //         break;
    //     }
    // }

    // let mut channel_id: Option<ChannelId> = None;

    // if is_managed {
    //     println!("[CMD; PLY]    {}", "Existing channel");
        
    //     // Have to repeat this shit to prevent borrowing lifetime problems
    //     let mut matching: Option<&ManagedChannel> = None;
    //     for channel in lock.values() {
    //         if channel.name == args[0] {
    //             matching = Some(channel);
    //             break;
    //         }
    //     }

    //     match matching {
    //         Some(channel) => channel_id = Some(channel.id),
    //         None => {}
    //     }

    //     // TODO Move user
    // } else {
    //     println!("[CMD; PLY]    {}", "New Channel");

    //     match discord.create_channel(server_id, &args[0], ChannelType::Voice) {
    //         Ok(Channel::Public(channel)) => {
    //             lock.insert(channel.id, ManagedChannel::new(channel.id, args[0].to_owned()));
    //             channel_id = Some(channel.id);
    //         },
    //         Ok(_) => {
    //             println!("[ERR; PLY]    Incorrect channel type created");
    //             return 403
    //         }
    //         Err(err) => {
    //             println!("[ERR; PLY]    Could not create channel; {:?}", err);
    //             return 400;
    //         }
    //     }
    // }

    let channel_name = args[0].clone();

    match create_managed_channel(discord, &message.author.id, channel_name) {
        Ok(channel_id) => {
            match discord.move_member_voice(server_id, message.author.id, channel_id) {
                Ok(_) => {}
                Err(err) => {
                    println!("[ERR; PLY]    Error moving user to channel; {:?}", err);
                    return 405;
                }
            }
        }
        Err(code) => {
            return code;
        }
    }

    0
}

/// Hooks into the VOICE_STATE_UPDATE event 
#[no_mangle]
pub extern fn hook_voice_state_update(discord: &Discord, _server_id: &ServerId, new_state: &VoiceState) -> u16 {
    let mut lock = match CHANNELS.lock() {
        Ok(channels) => channels,
        Err(_) => return 200
    };
    let mut state = match CHANNEL_CURSTATE.lock() {
        Ok(state) => state,
        Err(_) => return 201
    };
    let new_channel_id = match new_state.channel_id {
        Some(id) => id,
        None => ChannelId(0)
    };

    match state.get(&new_state.user_id) {
        Some(cur_channel_id) => {
            if new_channel_id != *cur_channel_id {
                // User changed channels, we don't care about self mute and other
                // voice state updates

                println!("[VSU; PLY]    User changed from channel {} to channel {}", *cur_channel_id, new_channel_id);

                let mut is_empty = false;

                match lock.get_mut(cur_channel_id) {
                    Some(channel) => {
                        if channel.users.len() < 2 {
                            is_empty = true;
                        } else {
                            channel.users.remove(&new_state.user_id);
                        }
                    }
                    None => {
                        // Not a managed channel
                        println!("[VSU; PLY]    Channel {} not managed", *cur_channel_id);
                    }
                };

                if is_empty {
                    // No users left, delete channel

                    match discord.delete_channel(*cur_channel_id) {
                        Ok(_) => {
                            lock.remove(cur_channel_id);
                        }
                        Err(err) => println!("[ERR; PLY]    VSU--Could not delete channel; {:?}", err)
                    }

                }
            }
        },
        None => {
            // Not currently tracking user
        }
    };

    state.insert(new_state.user_id, new_channel_id);

    match lock.get_mut(&new_channel_id) {
        Some(channel) => {
            // User moved into managed channel; add to channel user list
            
            channel.users.insert(new_state.user_id);
        }
        None => {
            // User moved somewhere we don't care about; do nothing
        }
    }

    0 // Successful
}