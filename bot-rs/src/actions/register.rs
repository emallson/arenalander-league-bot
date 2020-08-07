use crate::actions::league::current_league as get_league;
use crate::actions::token::generate_token;
use crate::deck_parser::Deck as ParsedDeck;
use crate::models::*;
use crate::schema::deck_contents::dsl::*;
use crate::schema::decks::dsl::*;
use crate::schema::users::dsl::*;
use anyhow::Result;
use std::collections::HashMap;
use chrono::Utc;
use regex::Regex;
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

/// Count the number of mana symbols of each color in the deck. Hybrid mana is
/// not counted.
fn count_symbols(decklist: &ParsedDeck) -> HashMap<char, i16> {
    let symbol_re = Regex::new(r"\{(?P<symbol>W|U|B|R|G)\}").unwrap();
    let mut counts = HashMap::new();
    for card_ in decklist {
        if let Some(ref cost) = card_.manacost {
            for cap in symbol_re.captures_iter(&cost) {
                *counts.entry(cap["symbol"].chars().nth(0).unwrap()).or_insert(0) += 1;
            }
        }
    }
    counts
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

    let counts = count_symbols(&parsed_deck);
    let now = Utc::now();
    // okay, at this point we have a user without an active deck, a known league, and a parsed, valid deck to insert. letsa go!
    let new_deck: Deck = insert_into(decks)
        .values((
            league.eq(current_league.id),
            owner.eq(user.id),
            creation_date.eq(now),
            symbols_w.eq(counts.get(&'W').cloned().unwrap_or(0)),
            symbols_u.eq(counts.get(&'U').cloned().unwrap_or(0)),
            symbols_b.eq(counts.get(&'B').cloned().unwrap_or(0)),
            symbols_r.eq(counts.get(&'R').cloned().unwrap_or(0)),
            symbols_g.eq(counts.get(&'G').cloned().unwrap_or(0)),
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
        .filter(owner.eq(user.id).and(league.eq(current_league.id)).and(active.eq(true)))
        .set((active.eq(false), resigned.eq(true)))
        .execute(conn)?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_symbol_count() {
        use uuid::Uuid;
        use crate::deck_parser::NormalizedCardEntry;
        use super::count_symbols;

        let deck = vec![
            NormalizedCardEntry {
                count: 1,
                manacost: Some("{W}{U}{W}{U}".to_owned()),
                uuid: Uuid::new_v4()
            },
            NormalizedCardEntry {
                count: 1,
                manacost: Some("{G}{G}{G}{W}{3}{R/B}".to_owned()),
                uuid: Uuid::new_v4(),
            }
        ];

        let counts = count_symbols(&deck);
        assert_eq!(counts[&'W'], 3);
        assert_eq!(counts[&'U'], 2);
        assert_eq!(counts.get(&'B'), None);
        assert_eq!(counts.get(&'R'), None);
        assert_eq!(counts[&'G'], 3);

    }
}
