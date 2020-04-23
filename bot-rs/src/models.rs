use chrono::{DateTime, Utc};
use super::schema::*;

#[allow(non_snake_case)]
#[derive(Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub discordid: i64,
}

#[derive(Queryable, Insertable)]
#[table_name="leagues"]
pub struct League {
    pub id: i32,
    pub title: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Queryable, Insertable, Associations)]
#[table_name="decks"]
#[belongs_to(parent="User", foreign_key="owner")]
#[belongs_to(parent="League", foreign_key="league")]
pub struct Deck {
    pub id: i32,
    pub owner: i32,
    pub league: i32,
    pub wins: i32,
    pub losses: i32,
    pub resigned: bool,
}

#[derive(Queryable, Insertable)]
#[table_name="card_names"]
pub struct CardName {
    pub id: i32,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Queryable, Insertable, Associations)]
#[table_name="deck_contents"]
#[belongs_to(parent="Deck", foreign_key="deck")]
#[belongs_to(parent="CardName", foreign_key="card")]
pub struct DeckContents {
    pub id: i32,
    pub deck: i32,
    pub card: i32,
}