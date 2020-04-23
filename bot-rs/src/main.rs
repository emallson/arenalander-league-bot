#![recursion_limit="256"]
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
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use chrono::Duration;

mod schema;
mod cards;
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

struct DbConn;

impl TypeMapKey for DbConn {
    type Value = Arc<Mutex<PgConnection>>;
}

struct PendingRegistrationSet;

impl TypeMapKey for PendingRegistrationSet {
    type Value = HashMap<UserId, DateTime<Utc>>;
}

struct Handler; 
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read();
        let pending_registrations = data.get::<PendingRegistrationSet>().unwrap();
        if msg.is_private() && !msg.author.bot && pending_registrations.contains_key(&msg.author.id) && (Utc::now() - pending_registrations[&msg.author.id]) <= Duration::hours(1) {
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

            {
                let conn = data.get::<DbConn>().unwrap();
                match deck_parser::parse_deck(&*conn.lock().unwrap(), &deck) {
                    Ok(_) => logged_dm(&ctx, &msg.author, "Deck successfully registered!"),
                    Err(e) => {
                        println!("Parse error: {:?} for decklist {}", e, deck);
                        logged_dm(&ctx, &msg.author, &format!("Unable to register deck: {}", e))
                    },
                };
            }
            drop(data);
            let mut data = ctx.data.write();
            let pending_registrations = data.get_mut::<PendingRegistrationSet>().unwrap();
            pending_registrations.remove(&msg.author.id);
        }
    }
}

fn main() {
    dotenv().expect("Unable to setup environment.");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    let conn = PgConnection::establish(&database_url)
        .expect("Unable to connect to database.");

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token to exist."), Handler)
        .expect("Error creating Discord Client");

    {
        let mut data = client.data.write();
        data.insert::<PendingRegistrationSet>(HashMap::new());
        data.insert::<DbConn>(Arc::new(Mutex::new(conn)));
    }
    client.with_framework(StandardFramework::new().configure(|c| c.prefix("!")).group(&LEAGUE_GROUP));

    if let Err(why) = client.start() {
        println!("An error occurred in the discord client: {:?}", why);
    }
}

#[command]
fn register(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();
    data.get_mut::<PendingRegistrationSet>().unwrap().insert(msg.author.id, Utc::now());
    msg.author.direct_message(&ctx, |m| {
        m.content("Please export your deck from MTG Arena and paste it here. If Discord asks, upload it as a text file.")
    })?;
    Ok(())
}