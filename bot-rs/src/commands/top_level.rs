use crate::{
    actions, DbConn, PendingRegistrationSet, PendingResignationSet, ACTIVE_CHECK, BASE_URL,
};
use chrono::{Duration, Utc};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
use serenity::prelude::*;
use std::collections::HashMap;

use super::util::fmt_command;

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
            msg.channel_id.say(&ctx.http, format!("You have a deck registered in this league already. If you would like to register with another deck, please resign from the league with the `{}` command.", fmt_command("resign")))?;
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
    if let Err(e) = msg.author.direct_message(&ctx, |m| {
        m.content("Please export your deck from MTG Arena and paste it here. If Discord asks, upload it as a text file.")
    }) {
        warn!("Unable to DM user for registration: {:?}", e);
        let response = MessageBuilder::new()
            .mention(&msg.author)
            .push("I wasn't able to send you a DM. Do you have DMs from server members allowed?")
            .build();
        msg.channel_id.say(
            &ctx.http,
            response
        )?;
    }
    Ok(())
}

#[command]
#[description("Get details of the currently active league.")]
fn league(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap();
    let league = actions::league::current_league(&*conn.lock().unwrap())?;

    let message = match league {
        Some(league) => format!("The {} League is currently active. It began on {} and runs until {}. To register a deck for the league, use `{}`. League standings can be viewed at {}/standings", league.title, league.start_date.format("%e %B %Y"), (league.end_date - Duration::seconds(1)).format("%r %Z on %e %B %Y"), fmt_command("register"), BASE_URL),
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
    {
        let conn = data.get::<DbConn>().unwrap().lock().unwrap();
        let unc_match = actions::matches::unconfirmed_match(&*conn, &msg.author)?;

        if unc_match.is_some() {
            msg.channel_id.say(&ctx.http, "You cannot resign with match confirmation pending. Resolve the pending match, then try again.")?;
            return Ok(());
        }
    }

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
        msg.channel_id.say(&ctx.http, format!("Are you sure you want to resign? Your record will remain as-is and your decklist will become public. If you would like to resign, use the `{}` command again.", fmt_command("resign")))?;
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
            if opps.is_empty() {
                msg.channel_id.say(
                    &ctx.http,
                    "You have not played any matches in the current league.",
                )?;
            } else {
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        // should only ever be 1
                        let unconfirmed = opps
                            .iter()
                            .filter(|opp| !opp.confirmed)
                            .map(|opp| opp.name.clone())
                            .collect::<Vec<_>>()
                            .join("\n");

                        let mut inactive_map = opps.iter().filter(|opp| opp.confirmed && !opp.active)
                            .map(|opp| (opp.discordid, opp.name.clone())).collect::<HashMap<_, _>>();

                        // only includes confirmed & active
                        let confirmed = opps
                            .into_iter()
                            .filter(|opp| opp.confirmed && opp.active)
                            .map(|opp| {
                                // if we encounter an active and confirmed match against an opponent, they cannot also be inactive
                                inactive_map.remove(&opp.discordid);
                                opp.name
                            })
                            .collect::<Vec<_>>()
                            .join("\n");

                        let inactive = inactive_map.into_iter().map(|(_, val)| val).collect::<Vec<_>>().join("\n");

                        e.title("Opponents in Current League")
                            .description("You have already played against these individuals, and cannot replay them during your current run unless they are listed as eligible for rematch.");
                        if !confirmed.is_empty() {
                            e.field("Confirmed", confirmed, true);
                        }

                        if !unconfirmed.is_empty() {
                            e.field("Unconfirmed", unconfirmed, true);
                        }

                        if !inactive.is_empty() {
                            e.field("Eligible for Rematch", inactive, true);
                        }

                        e
                    });
                    m
                })?;
            }
        }
        Ok(None) => {
            msg.channel_id.say(
                &ctx.http,
                "You have not played any matches in the current league.",
            )?;
        }
        Err(err) => {
            error!("Error processing !opponents: {:?}", err);
            msg.channel_id
                .say(&ctx.http, "Unable to retrieve opponent list.")?;
        }
    }
    Ok(())
}
