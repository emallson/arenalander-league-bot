use diesel::prelude::*;
use diesel::PgConnection;
use diesel::{update, insert_into, delete};
use crate::schema::matches::dsl::*;
use crate::models::{Deck, Match, Dispute};
use chrono::Utc;
use anyhow::Result;
use thiserror::Error;
use serenity::model::user::User as SerenityUser;
use serenity::model::id::UserId;
use super::lookup::{lookup_user, lookup_deck, lookup_match};
use uuid::Uuid;
use crate::actions::token::generate_token;

#[derive(Error, Debug)]
pub enum MatchError {
    #[error("A match between these players already exists in this league.")]
    MatchExists,
    #[error("An unconfirmed match from this user is pending.")]
    MatchPending,
}

pub fn report_match(conn: &PgConnection, winner: &SerenityUser, loser: &SerenityUser, wins: u32, losses: u32) -> Result<()> {
    let old_match = lookup_match(conn, winner, loser)?; 

    if old_match.is_some() {
        return Err(MatchError::MatchExists.into());
    }


    let winner_deck= lookup_deck(conn, winner)?.unwrap();
    let unconfirmed_match: Option<Match> = matches.filter(winning_deck.eq(winner_deck.id).and(confirmed.eq(false))).get_result(conn).optional()?;

    if unconfirmed_match.is_some() {
        return Err(MatchError::MatchPending.into());
    }

    let loser_deck = lookup_deck(conn, loser)?.unwrap();


    insert_into(matches)
        .values((winning_deck.eq(winner_deck.id), losing_deck.eq(loser_deck.id), winner_wins.eq(wins as i32), loser_wins.eq(losses as i32), date.eq(Utc::now())))
        .execute(conn)?;
    Ok(())
}

// todo: return value should be a struct here
pub fn confirm_match(conn: &PgConnection, loser: &SerenityUser) -> Result<Option<(Match, UserId, bool, bool, i32, Uuid, i32, Uuid)>> {
    use crate::schema::decks::dsl::*;
    let loser_deck = lookup_deck(conn, loser)?.unwrap();

    let res: Option<Match> = update(matches)
        .filter(losing_deck.eq(loser_deck.id).and(confirmed.eq(false)))
        .set(confirmed.eq(true))
        .get_result(conn).optional()?;

    if let Some(match_) = res {
        const MAX_MATCHES: i64 = 5;

        let loser_done = count_matches(conn, &loser_deck)? >= MAX_MATCHES;
        if loser_done {
            update(decks)
                .filter(id.eq(loser_deck.id))
                .set(active.eq(false))
                .execute(conn)?;
        }

        let winner_deck: Deck = decks.filter(id.eq(match_.winning_deck)).get_result(conn)?;
        let winner = {
            use crate::schema::users::dsl::*;
            UserId(users.select(discordid).filter(id.eq(winner_deck.owner)).get_result::<i64>(conn)? as u64)
        };

        let winner_done = count_matches(conn, &winner_deck)? >= MAX_MATCHES;
        if winner_done {
            update(decks)
                .filter(id.eq(loser_deck.id))
                .set(active.eq(false))
                .execute(conn)?;
        }

        let winner_token = generate_token(conn, loser_deck.id)?;
        let loser_token = generate_token(conn, winner_deck.id)?;
        Ok(Some((match_, winner, winner_done, loser_done, winner_deck.id, winner_token, loser_deck.id, loser_token)))
    } else {
        Ok(None)
    }
}

fn count_matches(conn: &PgConnection, deck: &Deck) -> Result<i64> {
    matches.filter(confirmed.eq(true).and(winning_deck.eq(deck.id).or(losing_deck.eq(deck.id))))
        .count()
        .get_result(conn).map_err(|e| e.into())
}

pub fn dispute_match(conn: &PgConnection, dsp: &SerenityUser, opponent: &SerenityUser, explanation: &str) -> Result<Option<Dispute>> {
    use crate::schema::disputes::dsl::*;
    let match_ = lookup_match(conn, dsp, opponent)?;

    if match_.is_none() {
        return Ok(None);
    }

    let u_disputer = lookup_user(conn, dsp)?;
    let match_ = match_.unwrap();
    insert_into(disputes)
        .values((matchid.eq(match_.id), disputer.eq(u_disputer.id), note.eq(explanation), date.eq(Utc::now())))
        .get_result(conn).map(Option::Some).map_err(|e| e.into())
}

pub fn undo_match(conn: &PgConnection, user: &SerenityUser) -> Result<Option<Match>> {
    let deck = lookup_deck(conn, user)?;

    if deck.is_none() {
        return Ok(None);
    }

    delete(matches)
        .filter(winning_deck.eq(deck.unwrap().id).and(confirmed.eq(false)))
        .get_result(conn).optional().map_err(|e| e.into())
}