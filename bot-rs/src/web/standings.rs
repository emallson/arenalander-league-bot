use crate::models::{Deck, DeckRecord, User};
use actix_web::{
    get, web, HttpResponse, Responder, Result as WebResult, Scope,
};
use anyhow::Result;
use askama::Template;
use diesel::prelude::*;
use diesel::PgConnection;
use super::DbPool;

fn get_standings(
    conn: &PgConnection,
    league_id: Option<i32>,
) -> Result<Vec<(Deck, User, DeckRecord)>> {
    use crate::actions::league::current_league;
    use crate::schema::deck_records::dsl::deck_records;
    use crate::schema::decks::dsl::*;
    use crate::schema::users::dsl::users;

    let league_ = if let Some(league_id) = league_id {
        use crate::schema::leagues::dsl::*;
        leagues.filter(id.eq(league_id)).get_result(conn)?
    } else {
        let league_ = current_league(conn)?;

        if league_.is_none() {
            return Ok(vec![]);
        }

        league_.unwrap()
    };

    let current_decks: Vec<(Deck, User, DeckRecord)> = decks
        .inner_join(users)
        .inner_join(deck_records)
        .filter(league.eq(league_.id))
        .get_results(conn)?;

    Ok(current_decks)
}

#[derive(Template)]
#[template(path = "standings.html")]
struct Standings {
    contents: Vec<(Deck, User, DeckRecord)>,
}

#[get("")]
async fn standings(pool: web::Data<DbPool>) -> WebResult<impl Responder> {
    let conn = pool.get().expect("Unable to get DB connection");
    let contents = web::block(move || get_standings(&conn, None))
        .await
        .map_err(|e| {
            error!("Unable to retrieve standings: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(Standings { contents }.render().unwrap()))
}

#[get("/{id}")]
async fn standings_for(pool: web::Data<DbPool>, path: web::Path<(i32,)>) -> WebResult<impl Responder> {
    let conn = pool.get().expect("Unable to get DB connection");
    let contents = web::block(move || get_standings(&conn, Some(path.0)))
        .await
        .map_err(|e| {
            error!("Unable to retrieve standings: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(Standings { contents }.render().unwrap()))
}

pub fn service() -> Scope {
    web::scope("/standings")
        .service(standings)
        .service(standings_for)
}