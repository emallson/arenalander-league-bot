use crate::models::{Deck, User, DeckRecord};
use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result as WebResult};
use anyhow::Result;
use askama::Template;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

fn get_standings(conn: &PgConnection, league_id: Option<i32>) -> Result<Vec<(Deck, User, DeckRecord)>> {
    use crate::actions::league::current_league;
    use crate::schema::decks::dsl::*;
    use crate::schema::users::dsl::users;
    use crate::schema::deck_records::dsl::deck_records;

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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("LOCATION", "/standings")
        .finish()
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/standings")]
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

#[get("/deck/{id}")]
async fn deck(_pool: web::Data<DbPool>, _id: web::Path<(i32,)>) -> WebResult<impl Responder> {
    Ok(HttpResponse::Ok().body("Not Yet Implemented. Sorry!"))
}

pub async fn build_web_server() -> Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let db_url = env::var("DATABASE_URL").unwrap();

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(fs::Files::new("/public", "resources/public"))
            .service(index)
            .service(standings)
            .service(deck)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
