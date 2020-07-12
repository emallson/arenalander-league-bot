#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use anyhow::Result;
use chrono::prelude::*;
use chrono::{Duration, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serenity::client::Client;
use serenity::framework::standard::{
    help_commands,
    macros::{check, help},
    Args, CheckResult, CommandGroup, CommandOptions, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::user::User;
use serenity::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::{Arc, Mutex};

mod actions;
mod commands;
mod deck_parser;
mod mana_parser;
mod models;
mod schema;
mod web;

const BASE_URL: &str = "http://gladiator.emallson.net";

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read();
        let pending_registrations = data.get::<PendingRegistrationSet>().unwrap();
        if msg.is_private()
            && !msg.author.bot
            && pending_registrations.contains_key(&msg.author.id)
            && (Utc::now() - pending_registrations[&msg.author.id]) <= Duration::hours(1)
        {
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
                let conn = data.get::<DbConn>().unwrap().lock().unwrap();
                match deck_parser::parse_deck(&*conn, &deck)
                    .and_then(|deck| actions::register::register_deck(&*conn, &msg.author, deck))
                {
                    Ok((deck, token)) => {
                        let message = format!(
                            "Deck successfully registered! You can view it at {}",
                            deck_url(deck.id, Some(token))
                        );
                        logged_dm(&ctx, &msg.author, &message)
                    }
                    Err(e) => {
                        error!("Parse error: {:?} for decklist {}", e, deck);
                        logged_dm(
                            &ctx,
                            &msg.author,
                            &format!("Unable to register deck: {}", e),
                        )
                    }
                };
            }
            drop(data);
            let mut data = ctx.data.write();
            let pending_registrations = data.get_mut::<PendingRegistrationSet>().unwrap();
            pending_registrations.remove(&msg.author.id);
        }
    }
}


fn logged_dm(ctx: &Context, user: &User, message: &str) {
    match user.dm(ctx, |m| m.content(message)) {
        Ok(_) => {}
        Err(e) => error!("Sending DM failed with error {:?}", e),
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
struct PendingResignationSet;

impl TypeMapKey for PendingResignationSet {
    type Value = HashMap<UserId, DateTime<Utc>>;
}

fn deck_url(id: i32, token: Option<uuid::Uuid>) -> String {
    match token {
        Some(tok) => format!("{}/deck/{}?token={}", BASE_URL, id, tok),
        None => format!("{}/deck/{}", BASE_URL, id)
    }
}

fn pre_setup() -> Result<(log::LevelFilter, Box<dyn log::Log>)> {
    match dotenv() {
        Ok(_) => {}
        Err(e) => warn!("dotenv failed to set environment: {:?}", e),
    };

    Ok(fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .level_for("serenity", log::LevelFilter::Info)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("rustls", log::LevelFilter::Warn)
        .chain(std::io::stderr())
        .into_log())
}

fn connect_db() -> Result<PgConnection> {
    let database_url = env::var("DATABASE_URL")?;
    PgConnection::establish(&database_url).map_err(|e| e.into())
}

fn build_discord_client() -> std::thread::JoinHandle<Result<()>> {
    std::thread::spawn(|| {
        let conn = connect_db()?;
        let mut client = Client::new(
            &env::var("DISCORD_TOKEN").expect("token to exist."),
            Handler,
        )
        .expect("Error creating Discord Client");

        {
            let mut data = client.data.write();
            data.insert::<PendingRegistrationSet>(HashMap::new());
            data.insert::<PendingResignationSet>(HashMap::new());
            data.insert::<DbConn>(Arc::new(Mutex::new(conn)));
        }

        let prefix = env::var("PREFIX").unwrap_or_else(|_| "!".to_string());
        client.with_framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&prefix))
                .before(|_ctx, msg, cmd_name| {
                    info!(
                        "Received command {} (message: '{}' from {:?})",
                        cmd_name, msg.content, msg.author.name
                    );
                    true
                })
                .after(|_ctx, msg, cmd_name, result| {
                    if let Err(err) = result {
                        error!(
                            "Error while running {}: {:?} (message: '{}' from {:?})",
                            cmd_name, err, msg.content, msg.author
                        );
                    }
                })
                .on_dispatch_error(|ctx, msg, err| {
                    use serenity::framework::standard::DispatchError::*;
                    match err {
                        CheckFailed("Active", reason) => {
                            debug!("active check failed: {:?}", reason);
                            msg.channel_id.say(&ctx.http, "Only active league participants can use this command.")
                                .expect("Unable to respond to failed command");
                        }
                        NotEnoughArguments { min, given } => {
                            msg.channel_id.say(&ctx.http, format!("Too few arguments for command (expected: {}, actual: {})", min, given))
                                .expect("Unable to respond to command with too few arguments");
                        }
                        TooManyArguments { max, given } => {
                            msg.channel_id.say(&ctx.http, format!("Too many arguments for command (expected: {}, actual: {})", max, given))
                                .expect("Unable to respond to command with too few arguments");
                        }
                        _ => {
                            error!("Dispatch error: {:?}, author: {}", err, msg.author.name);
                        }
                    }
                })
                .unrecognised_command(|ctx, msg, cmd_name| {
                    match msg.channel_id.say(&ctx.http, format!("No command '{}'", cmd_name)) {
                        Ok(_) => {
                            debug!("Unknown command: {}", cmd_name);
                        },
                        Err(e) => {
                            error!("Error while responding to unknown command '{}': {}", cmd_name, e);
                        }
                    }
                })
                .group(&commands::top_level::LEAGUE_GROUP)
                .group(&commands::admin::LEAGUECONTROL_GROUP)
                .group(&commands::matches::LEAGUEMATCHGROUP_GROUP)
                .help(&HELP_CMD),
        );

        if let Err(why) = client.start() {
            error!("An error occurred in the discord client: {:?}", why);
        }

        Ok(())
    })
}

fn setup_sentry(
    filter: log::LevelFilter,
    logger: Box<dyn log::Log>,
) -> Result<Option<sentry::internals::ClientInitGuard>> {
    if let Ok(token) = env::var("SENTRY_TOKEN") {
            let guard = sentry::init(token);
            sentry::integrations::panic::register_panic_handler();
            let log_options = sentry::integrations::log::LoggerOptions {
                global_filter: Some(filter),
                ..Default::default()
            };
            sentry::integrations::log::init(Some(logger), log_options);

            Ok(Some(guard))
    } else {
        log::set_boxed_logger(logger)?;
        log::set_max_level(filter);
        Ok(None)
    }
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let (filter, logger) = pre_setup()?;

    let _sentry_guard = setup_sentry(filter, logger)?;

    let discord_handle = build_discord_client();

    web::build_web_server().await?;

    discord_handle.join().unwrap()?;

    Ok(())
}

#[help]
#[lacking_permissions = "hide"]
#[lacking_role = "hide"]
#[lacking_ownership = "hide"]
fn help_cmd(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

#[check]
#[name("Active")]
fn active_participant(
    ctx: &mut Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();
    actions::league::check_active(&*conn, &msg.author)
        .expect("Unable to check for league activity.")
        .into()
}
