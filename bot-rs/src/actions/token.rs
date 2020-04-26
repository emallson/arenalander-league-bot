use crate::schema::deck_view_tokens::dsl::*;
use anyhow::Result;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

pub fn generate_token(conn: &PgConnection, deck_id: i32) -> Result<Uuid> {
    insert_into(deck_view_tokens)
        .values(&deck.eq(deck_id))
        .returning(token)
        .get_result(conn)
        .map_err(|e| e.into())
}
