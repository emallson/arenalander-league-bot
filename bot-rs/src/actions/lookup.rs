use super::league::current_league;
use crate::models::{Deck, Match, User};
use anyhow::Result;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::PgConnection;
use serenity::model::user::User as SerenityUser;

pub fn lookup_user(conn: &PgConnection, user: &SerenityUser) -> Result<User> {
    use crate::schema::users::dsl::*;
    users
        .filter(discordid.eq(*user.id.as_u64() as i64))
        .get_result(conn)
        .map_err(|e| e.into())
}

/// Looks up the deck belonging to the given user from the current league.
pub fn lookup_deck(conn: &PgConnection, user: &SerenityUser) -> Result<Option<Deck>> {
    use crate::schema::decks::dsl::*;
    let lg = current_league(conn)?;

    let user = match lookup_user(conn, user) {
        Ok(user) => user,
        Err(err) => match err.downcast_ref::<DieselError>() {
            Some(DieselError::NotFound) => return Ok(None),
            Some(_other) => return Err(err),
            None => unreachable!(),
        },
    };
    decks
        .filter(owner.eq(user.id).and(league.eq(lg.id)).and(active.eq(true)))
        .get_result(conn)
        .optional()
        .map_err(|e| e.into())
}

/// Looks up the match between the two given users from the current league.
pub fn lookup_match(
    conn: &PgConnection,
    left: &SerenityUser,
    right: &SerenityUser,
) -> Result<Option<Match>> {
    use crate::schema::matches::dsl::*;
    let uleft = lookup_deck(conn, left)?;
    let uright = lookup_deck(conn, right)?;

    if uleft.is_none() || uright.is_none() {
        return Ok(None);
    }

    let (uleft, uright) = (uleft.unwrap(), uright.unwrap());

    let query = matches.filter(
        winning_deck
            .eq(uleft.id)
            .and(losing_deck.eq(uright.id))
            .or(winning_deck.eq(uright.id).and(losing_deck.eq(uleft.id))),
    );

    query.get_result(conn).optional().map_err(|e| e.into())
}
