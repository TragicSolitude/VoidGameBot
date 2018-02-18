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

#[no_mangle]
pub extern fn describe() -> u64 {
    feature::TEST
}

#[no_mangle]
pub extern fn main(discord: &Discord, message: &Message, args: Vec<String>) -> u16 {
    if args.len() < 1 {
        return 100;
    }

    println!("[CMD; PLA]    Message from User ID: {}", message.author.id);

    let get_user_server_id: Symbol<extern fn(&Discord, &UserId) -> Result<ServerId, u16>>
        = unsafe { LIB_COMMON.get(b"get_user_server_id").unwrap() };
    let create_managed_channel: Symbol<extern fn(&Discord, &UserId, String) -> Result<ChannelId, u16>>
        = unsafe { LIB_COMMON.get(b"create_managed_channel").unwrap() };
    let get_users_in_channel: Symbol<extern fn(&ChannelId) -> Result<Vec<UserId>, u16>>
        = unsafe { LIB_COMMON.get(b"get_users_in_channel").unwrap() };
    let get_user_channel_id: Symbol<extern fn(&UserId) -> Result<ChannelId, u16>>
        = unsafe { LIB_COMMON.get(b"get_user_channel_id").unwrap() };
    let server_id = match get_user_server_id(discord, &message.author.id) {
        Ok(id) => id,
        Err(code) => return code
    };
    let channel_name = args[0].clone();

    match get_user_channel_id(&message.author.id) {
        Ok(current_channel_id) => {
            match create_managed_channel(discord, &message.author.id, channel_name) {
                Ok(channel_id) => {
                    match get_users_in_channel(&current_channel_id) {
                        Ok(users) => {
                            for user_id in users {
                                match discord.move_member_voice(server_id, user_id, channel_id) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        println!("[ERR; PLA]    Error moving user to channel; {:?}", err);
                                    }
                                }
                            }
                        }
                        Err(code) => {
                            return code;
                        }
                    }
                }
                Err(code) => {
                    return code;
                }
            }
        }
        Err(code) => {
            return code;
        }
    }


    0
}