use super::lookup::lookup_deck;
use crate::models::League;
use crate::schema::leagues::dsl::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::{delete, insert_into};
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

pub fn delete_league(conn: &PgConnection, lid: i32) -> Result<usize> {
    delete(leagues)
        .filter(id.eq(lid))
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn list_leagues(conn: &PgConnection) -> Result<Vec<League>> {
    leagues.get_results(conn).map_err(|e| e.into())
}

pub fn current_league(conn: &PgConnection) -> Result<Option<League>> {
    let now = Utc::now();
    leagues
        .filter(start_date.lt(now).and(end_date.gt(now)))
        .first(conn)
        .optional()
        .map_err(|e| e.into())
}

/// Checks if the given user has an active deck in the given league.
pub fn check_active(conn: &PgConnection, discord_user: &SerenityUser) -> Result<bool> {
    let deck = lookup_deck(conn, discord_user)?;
    Ok(deck.is_some())
}
