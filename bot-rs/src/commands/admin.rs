use chrono::{Utc, Duration};
use chrono_tz::US::Pacific;
use chrono_english::{parse_date_string, Dialect};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, ArgError,
};
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
use serenity::prelude::*;

use crate::actions;
use crate::DbConn;
use super::matches::respond_parse_error;
use crate::actions::matches;


#[group]
#[prefix("admin")]
#[only_in(guilds)]
#[allowed_roles("Mod", "League Bot Doctor")]
#[commands(new_league, list_leagues, delete_league, finalize_league, confirm_match)]
pub(crate) struct LeagueControl;

#[command]
#[only_in(guilds)]
#[num_args(3)]
#[delimiters(", ")]
#[description("Define a new league. All times are in UTC")]
#[usage("<title>, <start-date>, <end-date>")]
#[example("April 2020, 1 April 2020, 1 May 2020")]
fn new_league(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();
    let title = args.single::<String>()?;
    let from = parse_date_string(args.single::<String>()?.as_str(), Utc::now().with_timezone(&Pacific), Dialect::Us)?;
    let to = parse_date_string(args.single::<String>()?.as_str(), Utc::now().with_timezone(&Pacific), Dialect::Us)?;

    let offset = Duration::hours(10);

    let _league = actions::league::create_league(&*conn.lock().unwrap(), title, (from + offset).with_timezone(&Utc), (to + offset).with_timezone(&Utc))?;

    msg.channel_id
        .say(&ctx.http, "League successfully created!")?;

    Ok(())
}

#[command]
#[only_in(guilds)]
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
#[only_in(guilds)]
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

#[command]
#[only_in(guilds)]
#[description("Set all league decks to inactive. Should ONLY be used on leagues that have ended.")]
#[num_args(1)]
#[usage("<id>")]
fn finalize_league(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let id = args.single::<i32>()?;
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();

    let count = actions::league::finalize_league(&*conn.lock().unwrap(), id)
        .expect("Unable to finalize league");
    let message = format!(
        "Finalized league {}, marking {} decks as inactive.",
        id, count
    );
    msg.channel_id.say(&ctx.http, &message)?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Confirm a match. Used when a user unexpectedly AWOLs without confirming.")]
#[usage("<loser>")]
#[example("@emallson")]
fn confirm_match(ctx: &mut Context, msg: &Message) -> CommandResult {
    let loser = if let Some(user) = msg.mentions.get(0) {
        user
    } else {
        respond_parse_error(ctx, msg, ArgError::Parse("not enough mentions."))?;
        return Ok(());
    };

    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();

    if let Some((_, winner, _, _)) = matches::confirm_match(&*conn.lock().unwrap(), ctx, loser)? {
        let res = MessageBuilder::new()
            .push("Match between ")
            .mention(&winner)
            .push(" and ")
            .mention(loser)
            .push(" has been manually confirmed. Each should receive a link to their opponent's deck.")
            .build();
        msg.channel_id.say(&ctx.http, res)?;
    } else {
        msg.channel_id.say(&ctx.http, "No pending match for this user found.")?;
    }

    Ok(())
}
