mod handler;

extern crate discord;

use discord::{Discord, State};
use discord::model::*;
use discord::ChannelRef;
use std::{env};
use handler::{Handler, Context};

const COMMAND_PREFIX: &str = "rs";

fn main() {
    let mut handler = Handler::new();
    handler.add("help", help);
    handler.add("test", test);

    let token = env::var("DISCORD_TOKEN").expect("lol set DISCORD_TOKEN environment variable");
    let discord = Discord::from_bot_token(&token).expect("couldnt login lol");

    'connect: loop {
        println!("connecting...");
        let (mut conn, ready) = discord.connect().expect("couldn't connect lol");
        let mut state = State::new(ready);
        println!("connected!");

        'event: loop {
            let event = match conn.recv_event() {
                Ok(event) => event,
                Err(err) => {
                    println!("[ws err {:?}", err);
                    if let discord::Error::WebSocket(..) = err {
                        continue 'connect;
                    }
                    if let discord::Error::Closed(..) = err {
                        break 'connect;
                    }
                    continue;
                }
            };

            state.update(&event);

            match event {
                Event::MessageCreate(msg) => {
                    on_message(&discord, &state, &handler, msg);
                }
                _ => {}
            }
        }
    }
}

fn on_message(discord: &Discord, state: &State, handler: &Handler, msg: Message) {
    if !msg.content.starts_with(COMMAND_PREFIX) {
        return;
    }

    let (server, channel) = match state.find_channel(msg.channel_id) {
        Some(ChannelRef::Public(server, channel)) => (server, channel),
        None | _ => {
            println!("channel not found");
            return;
        }
    };

    println!("[{} #{}] {}: {}", server.name, channel.name, msg.author.name, msg.content);

    let content = &msg.content[COMMAND_PREFIX.len()..];
    let split: Vec<&str> = content.split(' ').collect();
    let (_, args) = split.as_slice().split_at(1);
    let name = split[0].to_lowercase();

    if let Some(cmd) = handler.commands.get(&name) {
        let member = server.members.iter()
            .find(|m| m.user.id == msg.author.id)
            .expect("could not find member");

        let ctx = Context{
            discord: discord,
            state: state,
            server: server,
            channel: channel,
            member: member,
            message: &msg,
            args: args
        };

        cmd(ctx);
    }
}

pub fn help(ctx: Context) {
    let _ = ctx.reply("help me lol");
}

pub fn test(ctx: Context) {
    let _ = ctx.reply(format!("ur args: {:?}", ctx.args).as_ref());
}
