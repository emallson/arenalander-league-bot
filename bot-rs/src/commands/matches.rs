use serenity::framework::standard::{
    macros::{command, group},
    ArgError, Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use serenity::Result;

use crate::{actions, deck_url, logged_dm, DbConn, ACTIVE_CHECK};
use super::util::fmt_command;

#[group]
#[prefix("match")]
#[only_in(guilds)]
#[checks(Active)]
#[description("Commands for match reporting.")]
#[commands(report, undo, confirm, dispute)]
pub(crate) struct LeagueMatchGroup;

fn respond_parse_error<E: std::fmt::Display>(
    ctx: &mut Context,
    msg: &Message,
    err: ArgError<E>,
) -> Result<Message> {
    msg.channel_id.say(&ctx.http, match err {
        ArgError::Eos => format!("Not enough arguments for that command. If you're not sure how to use it, try `{}`.", fmt_command("help")),
        ArgError::Parse(err) => format!("Unable to parse command argument: {}", err),
        _ => "Unable to parse command argument: unknown parse error occurred".to_owned(),
    })
}

#[command]
#[only_in(guilds)]
#[aliases("results")]
#[description("Report match results")]
#[usage("@opponent <your wins> <opponent wins>")]
#[example("@emallson 2 0")]
#[min_args(3)]
fn report(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.channel_id.say(
            &ctx.http,
            "In order to report match results, please @mention your opponent.",
        )?;
        return Ok(());
    }

    let opponent = &msg.mentions[0];

    if msg.author.id == opponent.id {
        // shenanigans!
        msg.channel_id.say(
            &ctx.http,
            "Try as you might, you can never win a match against yourself.",
        )?;
        return Ok(());
    }

    if let Err(err) = args.trimmed().single::<String>() {
        respond_parse_error(ctx, msg, err)?;
        return Ok(());
    };

    // discord inserts a space after mentions. people frequently add another one after. serenity
    // handles that...poorly. hacky workaround
    if let Some("") = args.current() {
        args.advance();
    }

    let wins = match args.single::<u32>() {
        Ok(w) => w,
        Err(e) => {
            respond_parse_error(ctx, msg, e)?;
            return Ok(());
        }
    };

    let losses = match args.single::<u32>() {
        Ok(l) => l,
        Err(e) => {
            respond_parse_error(ctx, msg, e)?;
            return Ok(());
        }
    };

    // confirmation logic depends on winner reporting
    if wins < losses {
        msg.channel_id
            .say(&ctx.http, "Please have the winner report the match.")?;
        return Ok(());
    } else if wins < 2 {
        msg.channel_id.say(
            &ctx.http,
            "This league runs Best-of-3 Matches. Please complete a Bo3 match and report back.",
        )?;
        return Ok(());
    }

    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    match actions::matches::report_match(&*conn, &msg.author, opponent, wins, losses) {
        Ok(Some(_)) => {
            let response = MessageBuilder::new()
                .push(format!("You reported that you won {}-{} against {}. ", wins, losses, opponent.name))
                .mention(opponent)
                .push(format!(" please use `{}` if the results are correct. If there was an error entering your results, ", fmt_command("match confirm")))
                .mention(&msg.author)
                .push(format!(" can use `{}`.", fmt_command("match undo")))
                .build();
            msg.channel_id.say(&ctx.http, response)?;
        }
        Ok(None) => {
            msg.channel_id.say(&ctx.http, "Unable to record match results. Are both you and your opponent registered for the league?")?;
        }
        Err(e) => {
            error!("Unable to record match: {:?}", e);
            msg.channel_id
                .say(&ctx.http, format!("Unable to record match: {}", e))?;
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Confirm match results")]
fn confirm(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    match actions::matches::confirm_match(&*conn, &msg.author) {
        Ok(Some((
            _match,
            winner,
            winner_done,
            loser_done,
            winner_deck,
            winner_token,
            loser_deck,
            loser_token,
        ))) => {
            msg.channel_id
                .say(&ctx.http, "Thanks for confirming the match!")?;
            let winner = winner.to_user(&ctx.http)?;

            logged_dm(
                ctx,
                &msg.author,
                &format!(
                    "Match recorded. Here is your opponent's deck for confirmation: {}",
                    deck_url(winner_deck, Some(loser_token))
                ),
            );

            logged_dm(
                ctx,
                &winner,
                &format!(
                    "Match recorded. Here is your opponent's deck for confirmation: {}",
                    deck_url(loser_deck, Some(winner_token))
                ),
            );

            if loser_done {
                let response = MessageBuilder::new()
                    .mention(&msg.author)
                    .push(" has completed their league run!")
                    .build();
                msg.channel_id.say(&ctx.http, response)?;
            }
            if winner_done {
                let response = MessageBuilder::new()
                    .mention(&winner)
                    .push(" has completed their league run!")
                    .build();
                msg.channel_id.say(&ctx.http, response)?;
            }
        }
        Ok(None) => {
            msg.channel_id.say(
                &ctx.http,
                "Hmm... you don't seem to have an unconfirmed match reported.",
            )?;
        }
        Err(e) => {
            error!("Unable to confirm match: {:?}", e);
            msg.channel_id
                .say(&ctx.http, "Unable to confirm match due to internal error.")?;
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Undo match report. Use if you accidentally report the wrong results. This can only be done on unconfirmed matches and only by the player that submitted the match report.")]
fn undo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    match actions::matches::undo_match(&*conn, &msg.author) {
        Ok(Some(_match)) => {
            msg.channel_id
                .say(&ctx.http, "Your match has been removed.")?;
        }
        Ok(None) => {
            msg.channel_id.say(
                &ctx.http,
                "Hmm... you don't seem to have an unconfirmed match reported.",
            )?;
        }
        Err(e) => {
            error!("Unable to undo match: {:?}", e);
            msg.channel_id
                .say(&ctx.http, "Unable to undo match due to internal error.")?;
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Dispute match results. Please only do this if you and your opponent are unable to resolve things yourselves.")]
#[usage("@opponent <explanation>")]
#[min_args(0)]
fn dispute(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let conn = data.get::<DbConn>().unwrap().lock().unwrap();

    if msg.mentions.is_empty() {
        msg.channel_id.say(
            &ctx.http,
            "In order to dispute match results, please @mention your opponent.",
        )?;
        return Ok(());
    }

    let opponent = &msg.mentions[0];
    let _ = args.single::<String>().unwrap();

    match actions::matches::dispute_match(&*conn, &msg.author, opponent, args.rest()) {
        Ok(Some(_dispute)) => {
            msg.channel_id.say(
                &ctx.http,
                "Your dispute has been recorded. A moderator will reach out to you.",
            )?;
        }
        Ok(None) => {
            msg.channel_id.say(
                &ctx.http,
                "Hmm... you don't seem to have a match reported with that player.",
            )?;
        }
        Err(e) => {
            error!("Unable to dispute match: {:?}", e);
            msg.channel_id
                .say(&ctx.http, "Unable to dispute match due to internal error.")?;
        }
    }

    Ok(())
}
