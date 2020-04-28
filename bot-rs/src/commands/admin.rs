use chrono::{Utc};
use chrono_english::{parse_date_string, Dialect};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::DbConn;
use crate::actions;

#[group]
#[prefix("admin")]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
#[commands(new_league, list_leagues, delete_league)]
pub(crate) struct LeagueControl;

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

    msg.channel_id
        .say(&ctx.http, "League successfully created!")?;

    Ok(())
}

#[command]
#[description("List all existing leagues (includes inactive leagues).")]
fn list_leagues(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();
    let leagues =
        actions::league::list_leagues(&*conn.lock().unwrap()).expect("Unable to list leagues");

    let message = leagues
        .into_iter()
        .map(|l| {
            format!(
                "**{} League** (id: {}): {} to {}",
                l.title, l.id, l.start_date, l.end_date
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e.title("All Leagues").description(message));
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
