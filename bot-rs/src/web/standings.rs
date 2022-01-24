use super::DbPool;
use crate::actions::league::current_league;
use crate::models::{Deck, DeckRecord, League, User};
use actix_web::{get, web, HttpResponse, Responder, Result as WebResult, Scope};
use anyhow::Result;
use askama::Template;
use diesel::prelude::*;
use diesel::PgConnection;

fn get_league(conn: &PgConnection, league_id: Option<i32>) -> Result<Option<League>> {
    if let Some(league_id) = league_id {
        use crate::schema::leagues::dsl::*;
        leagues
            .filter(id.eq(league_id))
            .get_result(conn)
            .optional()
            .map_err(|e| e.into())
    } else {
        current_league(conn).map(|l| Some(l)).map_err(|e| e.into())
    }
}

/// Returns a vector of user names and league points, sorted in descending order by points.
fn get_user_standings(conn: &PgConnection, league_id: Option<i32>) -> Result<Vec<(String, usize)>> {
    use crate::schema::leaderboard::dsl::*;

    let lid = if let Some(lid) = league_id {
        lid
    } else {
        current_league(conn)?.id
    };

    let results: Vec<(String, i64, i64)> = leaderboard
        .select((name, wins, complete_runs))
        .filter(league.eq(lid))
        .get_results(conn)?;

    let mut results = results
        .into_iter()
        .map(|(u, w, runs)| (u, (w + runs) as usize))
        .filter(|&(_, points)| points > 0)
        .collect::<Vec<_>>();
    results.sort_by_key(|&(_, points)| std::cmp::Reverse(points));

    Ok(results)
}

fn get_standings(
    conn: &PgConnection,
    league_id: Option<i32>,
) -> Result<Vec<(Deck, User, DeckRecord)>> {
    use crate::schema::deck_records::dsl::{deck_records, match_wins, match_losses, game_wins, game_losses};
    use crate::schema::decks::dsl::*;
    use crate::schema::users::dsl::users;

    let league_ = get_league(conn, league_id)?;
    if league_.is_none() {
        return Ok(vec![]);
    }

    let current_decks: Vec<(Deck, User, DeckRecord)> = decks
        .inner_join(users)
        .inner_join(deck_records)
        .filter(league.eq(league_.unwrap().id))
        .filter(match_wins.gt(0).or(match_losses.gt(0)))
        .order_by((match_wins.desc(), match_losses.asc(), game_wins.desc(), game_losses.asc()))
        .get_results(conn)?;

    Ok(current_decks)
}

#[derive(Template)]
#[template(path = "standings.html")]
struct Standings {
    league: League,
    contents: Vec<(Deck, User, DeckRecord)>,
    leaders: Vec<(String, usize)>,
}

#[get("")]
async fn standings(pool: web::Data<DbPool>) -> WebResult<impl Responder> {
    let conn = pool.get().expect("Unable to get DB connection");
    let leaders_p = web::block(move || get_user_standings(&conn, None));

    let conn = pool.get().expect("Unable to get DB connection");
    let contents = web::block(move || get_standings(&conn, None))
        .await
        .map_err(|e| {
            error!("Unable to retrieve standings: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let leaders = leaders_p.await.map_err(|e| {
        error!("Unable to retrieve standings: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let conn = pool.get().expect("Unable to get DB connection");
    let league = web::block(move || current_league(&conn)).await?;

    Ok(HttpResponse::Ok()
       .content_type("text/html")
       .body(Standings { league, contents, leaders }.render().unwrap()))
}

#[get("/{id}")]
async fn standings_for(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> WebResult<impl Responder> {
    let conn = pool.get().expect("Unable to get DB connection");
    let lid = path.0;
    let leaders_p = web::block(move || get_user_standings(&conn, Some(lid)));

    let conn = pool.get().expect("Unable to get DB connection");
    let contents = web::block(move || get_standings(&conn, Some(lid)))
        .await
        .map_err(|e| {
            error!("Unable to retrieve standings: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    let leaders = leaders_p.await.map_err(|e| {
        error!("Unable to retrieve standings: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let conn = pool.get().expect("Unable to get DB connection");
    let league = web::block(move || get_league(&conn, Some(lid))).await?.unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(Standings { league, contents, leaders }.render().unwrap()))
}

pub fn service() -> Scope {
    web::scope("/standings")
        .service(standings)
        .service(standings_for)
}
