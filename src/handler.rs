extern crate discord;

use std::collections::HashMap;
use discord::{Discord, State};
use discord::model::*;

pub type Command = fn(ctx: Context);

pub struct Handler {
    pub commands: HashMap<String, Command>
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            commands: HashMap::new()
        }
    }

    pub fn add(&mut self, name: &str, cmd: Command) {
        self.commands.insert(name.to_string(), cmd);
    }
}

#[allow(dead_code)]
pub struct Context<'a> {
    pub discord: &'a Discord,
    pub state: &'a State,
    pub server: &'a LiveServer,
    pub channel: &'a PublicChannel,
    pub member: &'a Member,
    pub message: &'a Message,
    pub args: &'a [&'a str]
}

impl<'a> Context<'a> {
    pub fn reply(&self, msg: &str) -> discord::Result<Message> {
        self.discord.send_message(self.channel.id, msg, "", false)
    }
}