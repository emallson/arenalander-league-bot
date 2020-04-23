#[macro_use] extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use serenity::model::id::UserId;
use std::collections::HashSet;

mod schema;
mod models;
mod deck_parser;

#[group]
#[commands(register)]
struct League;

fn logged_dm(ctx: &Context, user: &User, message: &str) {
    match user.dm(ctx, |m| m.content(message)) {
        Ok(_) => {},
        Err(e) => println!("Sending DM failed with error {:?}", e),
    }
}

struct Handler {
    pending_registrations: HashSet<UserId>,
}
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.is_private() && !msg.author.bot {
            // handle decklists
            let deck = if msg.attachments.is_empty() {
                // try the message contents
                msg.content.to_owned()
            } else {
                // try the first attachment
                match msg.attachments[0].download() {
                    Ok(contents) => String::from_utf8_lossy(&contents).to_string(),
                    Err(_) => {
                        logged_dm(&ctx, &msg.author, "Unable to load attached decklist.");
                        return;
                    }
                }
            };

            println!("{}", deck);
            
            match deck_parser::parse_deck(&deck) {
                Ok(_) => logged_dm(&ctx, &msg.author, "Deck successfully registered!"),
                Err(e) => logged_dm(&ctx, &msg.author, &format!("Unable to register deck: {}", e)),
            };
        }
    }
}

fn main() {
    dotenv().expect("Unable to setup environment.");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    let conn = PgConnection::establish(&database_url)
        .expect("Unable to connect to database.");

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token to exist."), Handler {
        pending_registrations: HashSet::new(),
    })
        .expect("Error creating Discord Client");
    client.with_framework(StandardFramework::new().configure(|c| c.prefix("!")).group(&LEAGUE_GROUP));

    if let Err(why) = client.start() {
        println!("An error occurred in the discord client: {:?}", why);
    }
}

#[command]
fn register(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.author.direct_message(&ctx, |m| {
        m.content("Please export your deck from MTG Arena and paste it here.")
    })?;
    Ok(())
}