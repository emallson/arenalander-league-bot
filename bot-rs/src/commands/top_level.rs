use chrono::{Duration, Utc};
use serenity::framework::standard::{
    macros::{group, command},
    CommandResult, 
};
use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::{ACTIVE_CHECK, DbConn, actions, PendingRegistrationSet, BASE_URL, PendingResignationSet};

#[group]
#[commands(register, league, resign, opponents)]
pub(crate) struct League;

#[command]
#[description("Register a deck for the league. You have until the end of the current league to complete 5 matches. LeagueBot will DM you with registration instructions.")]
fn register(ctx: &mut Context, msg: &Message) -> CommandResult {
    {
        let data = ctx.data.read();
        let conn = data.get::<DbConn>().unwrap().lock().unwrap();
        let lg = actions::league::current_league(&*conn)?;

        if lg.is_none() {
            debug!("!register without an active league");
            msg.channel_id
                .say(&ctx.http, "There is currently active league.")?;
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
    data.get_mut::<PendingRegistrationSet>()
        .unwrap()
        .insert(msg.author.id, Utc::now());
    msg.channel_id.say(
        &ctx.http,
        &"The LeagueBot will send you a direct message with further instructions.",
    )?;
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
        Some(league) => format!("The {} League is currently active. It began on {} and runs until {}. To register a deck for the league, use `!register`. League standings can be viewed at {}/standings", league.title, league.start_date.format("%e %B %Y"), league.end_date.format("%e %B %Y"), BASE_URL),
        None => "There is not currently a league active.".to_string()
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &message) {
        error!("error in !league: {:?}", why);
    }

    Ok(())
}

#[command]
#[checks(Active)]
#[description(
    "Resign from the league. Your record will remain as-is, and your decklist will become public."
)]
fn resign(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write();
    let resignations = data.get_mut::<PendingResignationSet>().unwrap();

    if resignations.contains_key(&msg.author.id)
        && (Utc::now() - resignations[&msg.author.id]) <= Duration::minutes(15)
    {
        resignations.remove(&msg.author.id);
        let conn = data.get::<DbConn>().unwrap().lock().unwrap();
        match actions::register::resign(&*conn, &msg.author) {
            Ok(_) => {
                msg.channel_id
                    .say(&ctx.http, "You have been resigned from the league.")?;
            }
            Err(err) => {
                error!("Error processing resignation: {:?}", err);
                msg.channel_id
                    .say(&ctx.http, "There was an error processing your resignation.")?;
            }
        }
    } else {
        resignations.insert(msg.author.id, Utc::now());
        msg.channel_id.say(&ctx.http, "Are you sure you want to resign? Your record will remain as-is and your decklist will become public. If you would like to resign, use the !resign command again.")?;
    }
    Ok(())
}

#[command]
#[checks(Active)]
#[description("List the opponents you have already played during this league run.")]
fn opponents(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    match actions::matches::list_opponents(&*conn, &msg.author) {
        Ok(Some(opps)) => {
            msg.author.dm(&ctx.http, |m| {
                m.embed(|e| {
                    let desc = opps.into_iter().map(|(opp, confirmed)| format!("{}\t{}", opp, if confirmed { "Confirmed" } else { "Unconfirmed" })).collect::<Vec<_>>().join("\n");
                    e.title("Opponents in Current League")
                    .description(desc)
                });
                m
            })?;
        },
        Ok(None) => {
            msg.author.dm(&ctx.http, |m| m.content("You have not played any matches in the current league."))?;
        },
        Err(err) => {
            error!("Error processing !opponents: {:?}", err);
            msg.channel_id.say(&ctx.http, "Unable to retrieve opponent list.")?;
        }
    }
    Ok(())
}