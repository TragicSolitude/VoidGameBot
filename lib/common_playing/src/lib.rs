#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;

mod managed_channel;

use std::sync::Mutex;
use std::collections::HashMap;
use discord::Discord;
use discord::model::{ServerId, VoiceState, ChannelId, UserId, Channel, ChannelType};
use managed_channel::ManagedChannel;
use common_void::{feature, hook};

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

#[no_mangle]
pub extern fn get_user_server_id(discord: &Discord, user_id: &UserId) -> Result<ServerId, u16> {
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
                    println!("[ERR; LPL]   User is not in a public channel");
                    return Err(402);
                }
                Err(err) => {
                    println!("[ERR; LPL]   Error getting channel info; {:?}", err);
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
#[no_mangle]
pub extern fn create_managed_channel(discord: &Discord, user_id: &UserId, name: String) -> Result<ChannelId, u16> {
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
            println!("[LIB; LPL]    {}", "New Channel");

            let server_id = match get_user_server_id(discord, user_id) {
                Ok(id) => id,
                Err(_) => {
                    println!("[LIB; LPL]    Could not get user's current server id");
                    return Err(450)
                }
            };

            match discord.create_channel(server_id, &name, ChannelType::Voice) {
                Ok(Channel::Public(channel)) => {
                    lock.insert(channel.id, ManagedChannel::new(channel.id, name));
                    channel_id = Ok(channel.id);
                },
                Ok(_) => {
                    println!("[ERR; LPL]    Incorrect channel type created");
                    return Err(403);
                }
                Err(err) => {
                    println!("[ERR; LPL]    Could not create channel; {:?}", err);
                    return Err(400);
                }
            }
        }
    }

    channel_id
}

#[no_mangle]
pub extern fn get_users_in_channel(channel_id: &ChannelId) -> Result<Vec<UserId>, u16> {
    let state = match CHANNEL_CURSTATE.lock() {
        Ok(state) => state,
        Err(_) => return Err(201)
    };
    let mut users = Vec::new();


    for k in state.keys() {
        let v = state.get(&k).unwrap();
        if *v == *channel_id {
            users.push(*k)
        }
    }

    Ok(users)
}

#[no_mangle]
pub extern fn get_user_channel_id(user_id: &UserId) -> Result<ChannelId, u16> {
    let state = match CHANNEL_CURSTATE.lock() {
        Ok(state) => state,
        Err(_) => return Err(201)
    };
    
    match state.get(user_id) {
        Some(channel_id) => Ok(*channel_id),
        None => Err(300)
    }
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
                        Err(err) => println!("[ERR; LPL]    VSU--Could not delete channel; {:?}", err)
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