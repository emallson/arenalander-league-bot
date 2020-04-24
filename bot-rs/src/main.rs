#[macro_use] extern crate diesel;
#[macro_use] extern crate log;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    CommandGroup,
    CommandOptions,
    CheckResult,
    HelpOptions,
    Args,
    macros::{
        check,
        help,
        command,
        group
    },
    help_commands
};
use serenity::model::id::UserId;
use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use chrono::{Utc, Duration};
use chrono_english::{parse_date_string, Dialect};
use anyhow::Result;

mod schema;
mod models;
mod deck_parser;
mod actions;
mod web;

#[group]
#[commands(register, league, resign)]
struct League;

#[group]
#[prefix("match")]
#[only_in(guilds)]
#[checks(Active)]
#[commands(report, undo, confirm, dispute)]
struct LeagueMatchGroup;

#[group]
#[prefix("admin")]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
#[commands(new_league, list_leagues, delete_league)]
struct LeagueControl;

fn logged_dm(ctx: &Context, user: &User, message: &str) {
    match user.dm(ctx, |m| m.content(message)) {
        Ok(_) => {},
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
                let conn = data.get::<DbConn>().unwrap().lock().unwrap();
                match deck_parser::parse_deck(&*conn, &deck)
                    .and_then(|deck| actions::register::register_deck(&*conn, &msg.author, deck)) {
                    Ok(_) => logged_dm(&ctx, &msg.author, "Deck successfully registered!"),
                    Err(e) => {
                        error!("Parse error: {:?} for decklist {}", e, deck);
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

fn pre_setup() -> Result<()> {
    dotenv()?;

    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .level_for("serenity", log::LevelFilter::Info)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("rustls", log::LevelFilter::Warn)
        .chain(std::io::stderr())
        .apply().unwrap();


    Ok(())
}

fn connect_db() -> Result<PgConnection> {
    let database_url = env::var("DATABASE_URL")?;
    PgConnection::establish(&database_url).map_err(|e| e.into())
}

fn build_discord_client() -> std::thread::JoinHandle<Result<()>> {
    std::thread::spawn(|| {
        let conn = connect_db()?;
        let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token to exist."), Handler)
            .expect("Error creating Discord Client");

        {
            let mut data = client.data.write();
            data.insert::<PendingRegistrationSet>(HashMap::new());
            data.insert::<PendingResignationSet>(HashMap::new());
            data.insert::<DbConn>(Arc::new(Mutex::new(conn)));
        }
        client.with_framework(StandardFramework::new().configure(|c| c.prefix("!"))
            .before(|_ctx, msg, cmd_name| {
                info!("Received command {} (message: '{}' from {:?})", cmd_name, msg.content, msg.author.name);
                true
            })
            .after(|_ctx, msg, cmd_name, result| {
                if let Err(err) = result {
                    error!("Error while running {}: {:?} (message: '{}' from {:?})", cmd_name, err, msg.content, msg.author);
                }
            })
            .group(&LEAGUE_GROUP)
            .group(&LEAGUECONTROL_GROUP)
            .group(&LEAGUEMATCHGROUP_GROUP)
            .help(&HELP_CMD));

        if let Err(why) = client.start() {
            error!("An error occurred in the discord client: {:?}", why);
        }

        Ok(())
    })
}

#[actix_rt::main]
async fn main() -> Result<()> {
    pre_setup()?;

    let discord_handle = build_discord_client();

    web::build_web_server().await?;

    discord_handle.join().unwrap()?;

    Ok(())
}

#[help]
#[lacking_permissions = "Hide"]
fn help_cmd(ctx: &mut Context, msg: &Message, args: Args, help_options: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

#[command]
#[description("Register a deck for the league. You have until the end of the current league to complete 5 matches. LeagueBot will DM you with registration instructions.")]
fn register(ctx: &mut Context, msg: &Message) -> CommandResult {
    {
        let data = ctx.data.read();
        let conn = data.get::<DbConn>().unwrap().lock().unwrap();
        let lg = actions::league::current_league(&*conn)?;

        if lg.is_none() {
            debug!("!register without an active league");
            msg.channel_id.say(&ctx.http, "There is currently active league.")?;
            return Ok(());
        }

        let is_active = actions::league::check_active(&*conn, &msg.author)?;
        if is_active {
            debug!("!register by an active user");
            msg.channel_id.say(&ctx.http, "You have a deck registered in this league already. If you would like to register with another deck, please resign from the league with the !resign command.")?;
            return Ok(());
        }
    }

    // now we can actually try to register them
    let mut data = ctx.data.write();
    data.get_mut::<PendingRegistrationSet>().unwrap().insert(msg.author.id, Utc::now());
    msg.channel_id.say(&ctx.http, &"The LeagueBot will send you a direct message with further instructions.")?;
    msg.author.direct_message(&ctx, |m| {
        m.content("Please export your deck from MTG Arena and paste it here. If Discord asks, upload it as a text file.")
    })?;
    Ok(())
}

#[command]
#[description("Get details of the currently active league.")]
fn league(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();
    let league = actions::league::current_league(&*conn.lock().unwrap())?;

    let message = match league {
        Some(league) => format!("The {} league is currently active. It began at {} and runs until {}. To register a deck for the league, use !register.", league.title, league.start_date.format("%e %B %Y"), league.end_date.format("%e %B %Y")),
        None => "There is not currently a league active.".to_string()
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &message) {
        error!("error in !league: {:?}", why);
    }

    Ok(())
}

#[command]
#[num_args(3)]
#[delimiters(", ")]
#[description("Define a new league. All times are in UTC")]
#[usage("<title>, <start-date>, <end-date>")]
#[example("!new-league April 2020, 1 April 2020, 1 May 2020")]
fn new_league(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();
    let title = args.single::<String>()?;
    let from = parse_date_string(args.single::<String>()?.as_str(), Utc::now(), Dialect::Us)
        .expect("Unable to parse 'from' argument");
    let to = parse_date_string(args.single::<String>()?.as_str(), Utc::now(), Dialect::Us)
        .expect("Unable to parse 'from' argument");
    let _league = actions::league::create_league(&*conn.lock().unwrap(), title, from, to)
        .expect("Unable to create new league");

    msg.channel_id.say(&ctx.http, "League successfully created!")?;

    Ok(())
}

#[command]
#[description("List all existing leagues (includes inactive leagues).")]
fn list_leagues(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();
    let leagues = actions::league::list_leagues(&*conn.lock().unwrap())
        .expect("Unable to list leagues");

    let message = leagues.into_iter().map(|l| format!("**{} League** (id: {}): {} to {}", l.title, l.id, l.start_date, l.end_date)).collect::<Vec<_>>().join("\n");
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("All Leagues").description(message)
        });
        m
    })?;
    Ok(())
}

#[command]
#[description("Delete a league. All associated decks will exist, but be unassigned to any league.")]
#[num_args(1)]
#[usage("<id>")]
fn delete_league(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let id = args.single::<i32>()?;
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();

    let _count = actions::league::delete_league(&*conn.lock().unwrap(), id)
        .expect("Unable to delete league");
    let message = format!("Deleted league {}", id);
    msg.channel_id.say(&ctx.http, &message)?;

    Ok(())
}

#[check]
#[name("Active")]
fn active_participant(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();
    actions::league::check_active(&*conn, &msg.author).expect("Unable to check for league activity.").into()
}

#[command]
#[checks(Active)]
#[description("Resign from the league. Your record will remain as-is, and your decklist will become public.")]
fn resign(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();
    let resignations = data.get_mut::<PendingResignationSet>().unwrap();

    if resignations.contains_key(&msg.author.id) && (Utc::now() - resignations[&msg.author.id]) <= Duration::minutes(15) {
        resignations.remove(&msg.author.id);
        let conn = data.get::<DbConn>().unwrap().lock().unwrap();
        match actions::register::resign(&*conn, &msg.author) {
            Ok(_) => { msg.channel_id.say(&ctx.http, "You have been resigned from the league.")?; },
            Err(err) => {
                error!("Error processing resignation: {:?}", err);
                msg.channel_id.say(&ctx.http, "There was an error processing your resignation.")?;
            }
        }
    } else {
        resignations.insert(msg.author.id, Utc::now());
        msg.channel_id.say(&ctx.http, "Are you sure you want to resign? Your record will remain as-is and your decklist will become public. If you would like to resign, use the !resign command again.")?;
    }
    Ok(())
}

#[command]
#[aliases("results")]
#[description("Report match results")]
#[usage("@opponent <your wins> <opponent wins>")]
#[example("@GoblinMatron 0 2")]
#[num_args(3)]
fn report(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id.say(&ctx.http, "In order to report match results, please @mention your opponent.")?;
        return Ok(());
    }

    let opponent = &msg.mentions[0];

    if msg.author.id == opponent.id {
        // shenanigans!
        msg.channel_id.say(&ctx.http, "Try as you might, you can never win a match against yourself.")?;
        return Ok(());
    }

    let _ = args.single::<String>().unwrap();
    let wins = args.single::<u32>().unwrap();
    let losses = args.single::<u32>().unwrap();

    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    match actions::matches::report_match(&*conn, &msg.author, opponent, wins, losses) {
        Ok(_) => {
            let response = MessageBuilder::new()
                .push("Your match has been recorded, but ")
                .mention(opponent)
                .push(" needs to !match confirm or !match dispute the results. If you made a mistake entering your results, use !match undo")
                .build();
            msg.channel_id.say(&ctx.http, response)?;
        },
        Err(e) => {
            error!("Unable to record match: {:?}", e);
            msg.channel_id.say(&ctx.http, "Unable to record match.")?;
        }
    }

    Ok(())
}

#[command]
#[description("Confirm match results")]
fn confirm(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    match actions::matches::confirm_match(&*conn, &msg.author) {
        Ok((Some(_match), winner_done, loser_done)) => {
            msg.channel_id.say(&ctx.http, "Thanks for confirming the match!")?;

            if loser_done {
                let response = MessageBuilder::new()
                    .mention(&msg.author)
                    .push(" has completed their league run!")
                    .build();
                msg.channel_id.say(&ctx.http, response)?;
            }
            if let Some(winner_id) = winner_done {
                let winner = winner_id.to_user(&ctx.http)?;
                let response = MessageBuilder::new()
                    .mention(&winner)
                    .push(" has completed their league run!")
                    .build();
                msg.channel_id.say(&ctx.http, response)?;
            }
        },
        Ok((None, _, _)) => {
            msg.channel_id.say(&ctx.http, "Hmm... you don't seem to have an unconfirmed match reported.")?;
        },
        Err(e) => {
            error!("Unable to confirm match: {:?}", e);
            msg.channel_id.say(&ctx.http, "Unable to confirm match due to internal error.")?;
        }
    }
    Ok(())
}

#[command]
#[description("Undo match report. Use if you accidentally report the wrong results. This can only be done on unconfirmed matches and only by the player that submitted the match report.")]
fn undo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    match actions::matches::undo_match(&*conn, &msg.author) {
        Ok(Some(_match)) => {
            msg.channel_id.say(&ctx.http, "Your match has been removed.")?;
        },
        Ok(None) => {
            msg.channel_id.say(&ctx.http, "Hmm... you don't seem to have an unconfirmed match reported.")?;
        },
        Err(e) => {
            error!("Unable to undo match: {:?}", e);
            msg.channel_id.say(&ctx.http, "Unable to undo match due to internal error.")?;
        }
    }
    Ok(())
}

#[command]
#[description("Dispute match results. Please only do this if you and your opponent are unable to resolve things yourselves.")]
#[usage("@opponent <explanation>")]
#[min_args(0)]
fn dispute(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    if msg.mentions.is_empty() {
        msg.channel_id.say(&ctx.http, "In order to dispute match results, please @mention your opponent.")?;
        return Ok(());
    }

    let opponent = &msg.mentions[0];
    let _ = args.single::<String>().unwrap();

    match actions::matches::dispute_match(&*conn, &msg.author, opponent, args.rest()) {
        Ok(Some(_dispute)) => {
            msg.channel_id.say(&ctx.http, "Your dispute has been recorded. A moderator will reach out to you.")?;
        }
        Ok(None) => {
            msg.channel_id.say(&ctx.http, "Hmm... you don't seem to have a match reported with that player.")?;
        }
        Err(e) => {
            error!("Unable to dispute match: {:?}", e);
            msg.channel_id.say(&ctx.http, "Unable to dispute match due to internal error.")?;
        }
    }

    Ok(())
}