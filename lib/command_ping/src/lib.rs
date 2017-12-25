extern crate discord;

use discord::Discord;
use discord::model::Message;

#[no_mangle]
pub extern fn main(discord: &Discord, message: &Message, args: Vec<String>) -> i16 {
    discord.send_message(
        message.channel_id,
        "asdfasdfasdf",
        "",
        false
    );
    0
}