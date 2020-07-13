use actix_files as fs;
use actix_web::{
    get, App, HttpResponse, HttpServer, Responder, middleware,
};
use anyhow::Result;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

mod standings;
mod deck;
mod graphql;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("LOCATION", "/standings")
        .finish()
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
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Compress::default())
            .service(fs::Files::new("/public", "resources/public"))
            .service(index)
            .service(standings::service())
            .service(deck::service())
            .service(graphql::service())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
