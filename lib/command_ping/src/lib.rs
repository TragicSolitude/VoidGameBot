extern crate discord;

use discord::Discord;
use discord::model::Message;

#[no_mangle]
pub extern fn main(discord: &Discord, message: &Message, args: Vec<String>) -> i16 {
    if args.len() == 0 {
        match discord.send_message(message.channel_id, "pong", "", false) {
            Ok(_) => 0,
            Err(_) => 1
        }
    } else {
        match discord.send_message(message.channel_id, "Targeted pings not implemented yet :)", "", false) {
            Ok(_) => 0,
            Err(_) => 1
        }
    }
}