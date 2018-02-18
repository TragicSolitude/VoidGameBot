extern crate discord;

use std::collections::HashSet;
use discord::model::{UserId, ChannelId};

pub struct ManagedChannel {
    pub users: HashSet<UserId>,
    pub id: ChannelId,
    pub name: String
}

impl ManagedChannel {
    pub fn new(id: ChannelId, name: String) -> ManagedChannel {
        ManagedChannel {
            users: HashSet::new(),
            id: id,
            name: name
        }
    }
}