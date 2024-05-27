mod triggermap;

use std::{default, env};

use serenity::all::Trigger;
use serenity::async_trait;
use serenity::futures::stream::All;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use std::fs::File;
use serenity::Client;
use std::io::Read;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        let mut map = triggermap::xlsx_to_hashmap("triggersheet.xlsx");
        println!("spreadsheet loaded!");
        let msg_iterator = msg.content.split_whitespace();
    
        for word in msg_iterator {
            if let Some(response) = map.get(word.to_string()) {
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    //C:/Users/Patron/programming/rust/discord_bot/token.txt

    let mut file = File::open("token.txt")
                              .expect("Error opening file");
       
    let mut token: String = String::new();
    file.read_to_string(&mut token).expect("uhm token doesn't exist");
    // let intents = GatewayIntents::DIRECT_MESSAGES
    //     | GatewayIntents::MESSAGE_CONTENT;


    let mut client =
        Client::builder(&token, GatewayIntents::all()).event_handler(Handler).await.expect("Err creating client");

    if let Err(msg) = client.start().await {
        println!("Error: {:?}", msg);
    }
}