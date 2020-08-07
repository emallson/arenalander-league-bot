use super::schema::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub discordid: i64,
    pub name: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "leagues"]
pub struct League {
    pub id: i32,
    pub title: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Queryable, Insertable, Associations)]
#[table_name = "decks"]
#[belongs_to(parent = "User", foreign_key = "owner")]
#[belongs_to(parent = "League", foreign_key = "league")]
pub struct Deck {
    pub id: i32,
    pub league: Option<i32>,
    pub owner: i32,
    pub creation_date: DateTime<Utc>,
    pub resigned: bool,
    pub active: bool,
    pub symbols_w: i16,
    pub symbols_u: i16,
    pub symbols_b: i16,
    pub symbols_r: i16,
    pub symbols_g: i16,
}

#[derive(Queryable, Insertable, Associations)]
#[table_name = "matches"]
#[belongs_to(parent = "Deck", foreign_key = "winning_deck")]
// we can't do double belongs_to to the same parent, so winners it is. losers have to get looked up manually
pub struct Match {
    pub id: i32,
    pub date: DateTime<Utc>,
    pub winning_deck: i32,
    pub losing_deck: i32,
    pub winner_wins: i32,
    pub loser_wins: i32,
    pub confirmed: bool,
}

// MTGJSON dump uses ridiculous types
#[derive(Queryable, Clone, PartialEq)]
pub struct Card {
    pub id: i64,
    pub name: String,
    pub setcode: String,
    pub number: String,
    pub isarena: i64,
    pub scryfalloracleid: Uuid,
    pub manacost: Option<String>,
    pub types: String,
    pub convertedmanacost: f64,
    pub uuid: Uuid,
}

#[derive(Queryable, Clone, PartialEq)]
pub struct CardName {
    pub id: i64,
    pub language: String,
    pub name: String,
    pub uuid: Uuid,
    pub scryfalloracleid: Uuid,
}

#[allow(non_snake_case)]
#[derive(Queryable, Insertable, Associations)]
#[table_name = "deck_contents"]
#[belongs_to(parent = "Deck", foreign_key = "deck")]
pub struct DeckContents {
    pub id: i32,
    pub deck: i32,
    pub card: Uuid,
    pub count: i32,
}

#[derive(Queryable, Associations)]
#[belongs_to(parent = "Match", foreign_key = "matchid")]
#[belongs_to(parent = "User", foreign_key = "disputer")]
pub struct Dispute {
    pub id: i32,
    pub matchid: i32,
    pub disputer: i32,
    pub date: DateTime<Utc>,
    pub resolve: bool,
    pub note: String,
}

#[derive(Queryable, Associations)]
#[belongs_to(parent = "Deck", foreign_key = "id")]
#[table_name = "deck_records"]
pub struct DeckRecord {
    pub id: i32,
    pub match_wins: i64,
    pub match_losses: i64,
    pub game_wins: i64,
    pub game_losses: i64,
}

#[derive(Queryable, Associations)]
#[table_name = "deck_view_tokens"]
#[belongs_to(parent = "Deck", foreign_key = "deck")]
pub struct DeckViewToken {
    pub id: i32,
    pub deck: i32,
    pub token: Uuid,
}
