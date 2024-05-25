mod triggermap;

use std::{default, env};

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
        } else if msg.content == "?ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "ping this ping that how about you go ping some bitches instead").await {
                println!("Error giving message: {:?}", why);
            }
        } else if contains_keywords_ignorecase(msg.content, &["viper", "lfg"]) {
            if let Err(why) = msg.channel_id.say(&ctx.http, "LFG VIPER MENTIONED").await {
                println!("Error giving message: {:?}", why);
            }
        } else if contains_keywords_ignorecase(msg.content, &["drg", "deep rock galactic"]) {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Did I hear a rock and stone?").await {
                println!("Error giving message: {:?}", why);
            }
        } else if msg.content.contains("lesbian") == true {
            if let Err(why) = msg.channel_id.say(&ctx.http, "O_O omg...women...pretty...").await {
                println!("Error giving message: {:?}", why);
            }
        } else if msg.content.contains("hipster") == true {
            if let Err(why) = msg.channel_id.say(&ctx.http, "hipster propaganda is the only good propaganda").await {
                println!("Error giving message: {:?}", why);
            }
        } else if msg.content.contains("monsoon") == true {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Wait...you mean THE monsoonification on twitch dot tv? No. 1 Driller NA?").await {
                println!("Error giving message: {:?}", why);
            }
        } else if msg.content.contains("dang") == true {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Dang... that's crazy...").await {
                println!("Error giving message: {:?}", why);
            }
        } else if msg.content.contains("worry") == true {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Don't worry about it >:3").await {
                println!("Error giving message: {:?}", why);
            }
        } else if msg.content.contains("downpour") && msg.content.contains("shut up") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Don't worry about it >:3").await {
                println!("Error giving message: {:?}", why);
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