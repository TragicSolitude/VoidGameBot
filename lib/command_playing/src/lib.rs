#[macro_use]
extern crate lazy_static;
extern crate discord;
extern crate common_void;
extern crate libloading;

use discord::Discord;
use discord::model::{Message, ServerId, ChannelId, UserId};
use libloading::{Library, Symbol};
use common_void::feature;

lazy_static! {
    static ref LIB_COMMON: Library = Library::new("plugins/libcommon_playing.so").unwrap();
}

/// Plugin feature usage description; gets called at link-time and returns a
/// bitmask of all features used by this plugin
#[no_mangle]
pub extern fn describe() -> u64 {
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

    println!("[CMD; PLY]    Message from User ID: {}", message.author.id);

    let get_user_server_id: Symbol<extern fn(&Discord, &UserId) -> Result<ServerId, u16>>
        = unsafe { LIB_COMMON.get(b"get_user_server_id").unwrap() };
    let create_managed_channel: Symbol<extern fn(&Discord, &UserId, String) -> Result<ChannelId, u16>>
        = unsafe { LIB_COMMON.get(b"create_managed_channel").unwrap() };
    let server_id = match get_user_server_id(discord, &message.author.id) {
        Ok(id) => id,
        Err(code) => return code
    };
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