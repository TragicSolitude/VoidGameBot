#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;

mod managed_channel;

use std::sync::Mutex;
use std::collections::HashMap;
use discord::Discord;
use discord::model::{Message, ServerId, VoiceState, ChannelId, UserId};
use common_void::{feature, hook};
use managed_channel::ManagedChannel;

// How should I init these kind of things?
lazy_static! {
    static ref CHANNELS: Mutex<HashMap<ChannelId, ManagedChannel>> = Mutex::new(HashMap::new());
    static ref CHANNEL_CURSTATE: Mutex<HashMap<UserId, ChannelId>> = Mutex::new(HashMap::new());
}

/// Plugin feature usage description; gets called at link-time and returns a
/// bitmask of all features used by this plugin
#[no_mangle]
pub extern fn describe() -> u32 {
    feature::TEST | hook::VOICE_STATE_UPDATE
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
///
/// # Arguments
/// Standard VGB plugin arguments
///
/// * `discord` - A reference to the discord instance
/// * `message` - A reference to the received message instance
/// * `args`    - A vector of all parameters specified for the command
#[no_mangle]
pub extern fn main(discord: &Discord, message: &Message, args: Vec<String>) -> u8 {
    if args.len() < 1 {
        return 100;
    }

    let lock = match CHANNELS.lock() {
        Ok(channels) => channels,
        Err(_) => return 200
    };

    // TODO Check to see if user is in a public voice channel

    let mut matching: Option<&ManagedChannel> = None;
    for channel in lock.values() {
        if channel.name == args[0] {
           matching = Some(channel);
           break;
        }
    }

    match matching {
        Some(managed_channel) => {
            println!("[CMD; PLY]    {}", "Existing channel");
        }
        None => {
            println!("[CMD; PLY]    {}", "New Channel");
        }
    }

    0
}

/// Hooks into the VOICE_STATE_UPDATE event 
#[no_mangle]
pub extern fn hook_voice_state_update(server_id: &ServerId, state: &VoiceState) -> u8 {
    let mut lock = match CHANNELS.lock() {
        Ok(channels) => channels,
        Err(_) => return 200
    };
    let mut prev = match CHANNEL_CURSTATE.lock() {
        Ok(prev) => prev,
        Err(_) => return 201
    };
    let state_channel_id = match state.channel_id {
        Some(id) => id,
        None => ChannelId(0)
    };

    match prev.get(&state.user_id) {
        Some(channel_id) => {
            if state_channel_id != *channel_id {
                match lock.get_mut(channel_id) {
                    Some(channel) => {
                        channel.users.remove(&state.user_id);
                    }
                    None => {
                        // Do something if user doesn't exist here?
                    }
                };
            }
        },
        None => {
            prev.insert(state.user_id, state_channel_id);
        }
    };

    // Delete managed channels with nobody in them

    0
}