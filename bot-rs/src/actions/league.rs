use num_traits::cast::FromPrimitive;
use super::lookup::lookup_deck;
use crate::models::League;
use crate::schema::leagues::dsl::*;
use anyhow::Result;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::{delete, insert_into, update};
use serenity::model::user::User as SerenityUser;

pub fn create_league(
    conn: &PgConnection,
    ltitle: String,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Result<League> {
    insert_into(leagues)
        .values((title.eq(ltitle), start_date.eq(from), end_date.eq(to)))
        .get_result(conn)
        .map_err(|e| e.into())
}

/// Create a new monthly league. Assumes it does not already exist.
///
/// # Panics
///
/// If it is not possible to create a DateTime at 10th hour of the 1st of the month, or if the
/// start date has an invalid month associated with it.
pub fn create_monthly_league(conn: &PgConnection) -> Result<League> {
    let today = Utc::now();
    let start = today.with_day(1).unwrap().with_hour(10).unwrap();
    let mut d = today.naive_utc().date();
    let end = loop {
        let next = d.succ();
        if next.month() != d.month() {
            break d;
        }
        d = next;
    };
    let end = DateTime::from_utc(end.and_hms(10, 0, 0), Utc);

    create_league(conn, format!("{} {}", Month::from_u32(start.month()).unwrap().name(), start.year()), start, end)
}

pub fn delete_league(conn: &PgConnection, lid: i32) -> Result<usize> {
    delete(leagues)
        .filter(id.eq(lid))
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn list_leagues(conn: &PgConnection) -> Result<Vec<League>> {
    leagues.get_results(conn).map_err(|e| e.into())
}

pub fn current_league(conn: &PgConnection) -> Result<League> {
    let now = Utc::now();
    let league = leagues
        .filter(start_date.lt(now).and(end_date.gt(now)))
        .first(conn)
        .optional()?;

    if let Some(league) = league {
        Ok(league)
    } else {
        create_monthly_league(conn)
    }
}

/// Checks if the given user has an active deck in the given league.
pub fn check_active(conn: &PgConnection, discord_user: &SerenityUser) -> Result<bool> {
    let deck = lookup_deck(conn, discord_user)?;
    Ok(deck.is_some())
}

pub fn finalize_league(conn: &PgConnection, league_id: i32) -> Result<usize> {
    use crate::schema::decks::dsl::{active, decks, league};

    update(decks)
        .filter(league.eq(league_id))
        .set(active.eq(false))
        .execute(conn)
        .map_err(|e| e.into())
}
