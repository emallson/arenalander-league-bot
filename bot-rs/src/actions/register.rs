use crate::actions::league::current_league as get_league;
use crate::actions::token::generate_token;
use crate::deck_parser::Deck as ParsedDeck;
use crate::models::*;
use crate::schema::deck_contents::dsl::*;
use crate::schema::decks::dsl::*;
use crate::schema::users::dsl::*;
use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::{insert_into, update};
use serenity::model::user::User as SerenityUser;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("You already have an active league deck: https://example.com/decks/{0}")]
    AlreadyActiveDeck(i32),
    #[error("There is not currently an active league")]
    NoLeague,
}

pub fn register_deck(
    conn: &PgConnection,
    user: &SerenityUser,
    parsed_deck: ParsedDeck,
) -> Result<(Deck, Uuid)> {
    let user: User = insert_into(users)
        .values((discordid.eq(*user.id.as_u64() as i64), name.eq(&user.name)))
        .on_conflict(discordid)
        .do_update()
        .set(name.eq(&user.name))
        .get_result(conn)?;

    let current_league = get_league(conn)?;
    if current_league.is_none() {
        return Err(RegistrationError::NoLeague.into());
    }

    let current_league = current_league.unwrap();

    let active_deck: Option<Deck> = decks
        .filter(
            league
                .eq(current_league.id)
                .and(owner.eq(user.id))
                .and(active.eq(true)),
        )
        .first(conn)
        .optional()?;

    if let Some(d) = active_deck {
        return Err(RegistrationError::AlreadyActiveDeck(d.id).into());
    }

    let now = Utc::now();
    // okay, at this point we have a user without an active deck, a known league, and a parsed, valid deck to insert. letsa go!
    let new_deck: Deck = insert_into(decks)
        .values((
            league.eq(current_league.id),
            owner.eq(user.id),
            creation_date.eq(now),
        ))
        .get_result(conn)?;
    let contents = parsed_deck
        .into_iter()
        .map(|entry| {
            (
                deck.eq(new_deck.id),
                card.eq(entry.uuid),
                count.eq(entry.count as i32),
            )
        })
        .collect::<Vec<_>>();
    insert_into(deck_contents).values(&contents).execute(conn)?;

    generate_token(conn, new_deck.id).map(|uuid| (new_deck, uuid))
}

pub fn resign(conn: &PgConnection, user: &SerenityUser) -> Result<()> {
    let user: User = users
        .filter(discordid.eq(*user.id.as_u64() as i64))
        .get_result(conn)?;
    let current_league = get_league(conn)?;
    if current_league.is_none() {
        return Err(RegistrationError::NoLeague.into());
    }

    let current_league = current_league.unwrap();

    update(decks)
        .filter(owner.eq(user.id).and(league.eq(current_league.id)))
        .set((active.eq(false), resigned.eq(true)))
        .execute(conn)?;

    Ok(())
}
