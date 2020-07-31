use super::DbPool;
use crate::models::{Deck, DeckRecord, League, Match};
use actix_web::{web, Error as WebError, HttpResponse, Result as WebResult, Scope};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use uuid::Uuid;

pub struct Context {
    pool: DbPool,
}

impl juniper::Context for Context {}

struct Query;

// Card is special since we actually only have a subset of fields.
struct Card {
    name: String,
    scryfallid: Uuid,
    scryfalloracleid: Uuid,
    manacost: Option<String>,
    types: String,
    convertedmanacost: f64,
}

#[juniper::object]
impl Card {
    fn name(&self) -> &str {
        &self.name
    }

    fn scryfall_id (&self) -> Uuid {
        self.scryfallid
    }

    fn scryfall_oracle_id(&self) -> Uuid {
        self.scryfalloracleid
    }

    fn manacost(&self) -> Option<&String> {
        self.manacost.as_ref()
    }

    fn types(&self) -> &str {
        &self.types
    }

    fn cmc(&self) -> i32 {
        self.convertedmanacost as i32
    }
}

#[juniper::object]
impl DeckRecord {
    fn match_wins(&self) -> i32 {
        self.match_wins as i32
    }

    fn match_losses(&self) -> i32 {
        self.match_losses as i32
    }

    fn game_wins(&self) -> i32 {
        self.game_wins as i32
    }

    fn game_losses(&self) -> i32 {
        self.game_losses as i32
    }
}

struct DeckContents {
    count: i32,
    card: Card
}

#[juniper::object]
impl DeckContents {
    fn count(&self) -> i32 {
        self.count
    }

    fn card(&self) -> &Card {
        &self.card
    }
}

#[juniper::object(Context = Context)]
impl Deck {
    fn id(&self) -> i32 {
        self.id
    }

    /// ID of the player that created the deck.
    ///
    /// WARNING: These IDs may not remain the same value or even type!
    fn player(&self) -> i32 {
        self.owner
    }

    fn league(&self, ctx: &Context) -> Option<League> {
        use crate::schema::leagues::dsl::*;
        let conn = ctx.pool.get().unwrap();

        if let Some(lid) = self.league {
            Some(leagues.filter(id.eq(lid)).first(&conn).unwrap())
        } else {
            None
        }
    }

    fn creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }

    fn resigned(&self) -> bool {
        self.resigned
    }

    fn cards(&self, ctx: &Context) -> Option<Vec<DeckContents>> {
        let contents: Option<Vec<_>> = if self.active {
            None
        } else {
            use crate::schema::deck_contents::dsl::*;
            use crate::schema::cards::dsl::{cards, name, scryfallid, manacost, types, convertedmanacost, scryfalloracleid};

            let conn = ctx.pool.get().unwrap();

            Some(deck_contents
                 .filter(deck.eq(self.id))
                 .inner_join(cards.on(scryfalloracleid.eq(card)))
                 .select((count, name, scryfallid, scryfalloracleid, manacost, types, convertedmanacost))
                 .distinct_on(scryfalloracleid)
                 .get_results(&conn)
                 .unwrap())
        };

        contents.map(|c| c.into_iter().map(|(count, name, scryfallid, scryfalloracleid, manacost, types, convertedmanacost)| DeckContents {
            count,
            card: Card {
                name,
                scryfallid,
                scryfalloracleid,
                manacost,
                types,
                convertedmanacost
            }
        }).collect())
    }

    fn record(&self, ctx: &Context) -> DeckRecord {
        use crate::schema::deck_records::dsl::*;
        let conn = ctx.pool.get().unwrap();

        deck_records.filter(id.eq(self.id)).first(&conn).unwrap()
    }

    fn matches(&self, ctx: &Context) -> Vec<Match> {
        use crate::schema::matches::dsl::*;
        let conn = ctx.pool.get().unwrap();

        matches
            .filter(
                confirmed
                    .eq(true)
                    .and(winning_deck.eq(self.id).or(losing_deck.eq(self.id))),
            )
            .get_results(&conn)
            .unwrap()
    }
}

#[juniper::object(Context = Context)]
impl Match {
    fn id(&self) -> i32 {
        self.id
    }

    fn date(&self) -> DateTime<Utc> {
        self.date
    }

    fn winner(&self, ctx: &Context) -> Deck {
        use crate::schema::decks::dsl::*;
        let conn = ctx.pool.get().unwrap();

        decks.filter(id.eq(self.winning_deck)).first(&conn).unwrap()
    }

    fn loser(&self, ctx: &Context) -> Deck {
        use crate::schema::decks::dsl::*;
        let conn = ctx.pool.get().unwrap();

        decks.filter(id.eq(self.losing_deck)).first(&conn).unwrap()
    }

    fn winner_wins(&self) -> i32 {
        self.winner_wins
    }

    fn loser_wins(&self) -> i32 {
        self.loser_wins
    }
}

struct LeagueDecks {
    decks: Vec<Deck>,
    next_offset: Option<i32>,
    total: i32
}

#[juniper::object(Context=Context)]
impl LeagueDecks {
    fn decks(&self) -> &Vec<Deck> {
        &self.decks
    }

    fn next_offset(&self) -> Option<i32> {
        self.next_offset
    }

    fn total(&self) -> i32 {
        self.total
    }
}

#[juniper::object(Context = Context)]
impl League {
    fn id(&self) -> i32 {
        self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn start_date(&self) -> DateTime<Utc> {
        self.start_date
    }

    fn end_date(&self) -> DateTime<Utc> {
        self.end_date
    }

    /// Returns up to 25 decks at a time.
    fn decks(&self, ctx: &Context, offset: Option<i32>) -> LeagueDecks {
        use crate::schema::decks::dsl::*;
        let conn = ctx.pool.get().unwrap();

        let offset = offset.unwrap_or(0).max(0);

        let decks_ = decks.filter(league.eq(self.id)).limit(25).offset(offset as i64).get_results(&conn).unwrap();
        let total = decks.filter(league.eq(self.id)).count().get_result::<i64>(&conn).unwrap() as i32;
        LeagueDecks {
            decks: decks_,
            total,
            next_offset: if offset + 25 > total { None } else { Some(offset + 25) }
        }
    }
}

type PgConn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
fn find_card<E>(conn: &PgConn, expr: E) -> Option<Card>
where
    E: diesel::Expression<SqlType = diesel::sql_types::Bool>
        + diesel::expression::NonAggregate
        + diesel::expression::AppearsOnTable<crate::schema::cards::table>
        + diesel::query_builder::QueryId
        + diesel::query_builder::QueryFragment<diesel::pg::Pg>,
{
    let result = {
        use crate::schema::cards::dsl::*;

        cards
            .filter(expr)
            .select((name, scryfallid, scryfalloracleid, manacost, types, convertedmanacost))
            .first(conn)
            .optional()
            .expect("Unable to connect to DB for card lookup.")
    };

    result.map(
        |(name, scryfallid, scryfalloracleid, manacost, types, convertedmanacost)| Card {
            name,
            scryfallid,
            scryfalloracleid,
            manacost,
            types,
            convertedmanacost,
        },
    )
}

#[juniper::object(Context = Context)]
impl Query {
    /// Lookup a single card by its Scryfall Oracle ID or its Name.
    fn card(ctx: &Context, oracle_id: Option<Uuid>, name: Option<String>) -> Option<Card> {
        if let Some(id) = oracle_id {
            use crate::schema::cards::dsl::scryfalloracleid;
            let conn = ctx.pool.get().expect("Unable to get DB connection.");

            find_card(&conn, scryfalloracleid.eq(id))
        } else if let Some(name) = name {
            use crate::schema::cards::dsl::name as name_;
            let conn = ctx.pool.get().unwrap();

            find_card(&conn, name_.eq(name))
        } else {
            None
        }
    }

    fn leagues(ctx: &Context) -> Vec<League> {
        use crate::schema::leagues::dsl::leagues;
        let conn = ctx.pool.get().unwrap();

        leagues.get_results(&conn).unwrap()
    }

    fn league(ctx: &Context, id: i32) -> Option<League> {
        use crate::schema::leagues::dsl::{id as id_, leagues};
        let conn = ctx.pool.get().unwrap();

        leagues.filter(id_.eq(id)).first(&conn).optional().unwrap()
    }

    fn deck(ctx: &Context, id: i32) -> Option<Deck> {
        use crate::schema::decks::dsl::{decks, id as id_};
        let conn = ctx.pool.get().unwrap();

        decks.filter(id_.eq(id)).first(&conn).optional().unwrap()
    }

    /// Decks that contain the card with the given Scryfall Oracle ID.
    fn decks_with_card(ctx: &Context, id: Uuid) -> Vec<Deck> {
        use crate::schema::deck_contents::dsl::{card, deck, deck_contents};
        use crate::schema::decks::dsl::{active, decks, id as id_};

        let conn = ctx.pool.get().expect("Unable to get DB connection.");

        let deck_ids: Vec<i32> = deck_contents
            .filter(card.eq(id))
            .select(deck)
            .distinct()
            .get_results(&conn)
            .expect("Unable to connect to DB");

        decks
            .filter(id_.eq_any(deck_ids).and(active.eq(false)))
            .get_results(&conn)
            .expect("Unable to connect to DB")
    }
}

struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {}

type Schema = juniper::RootNode<'static, Query, Mutation>;

async fn graphql(
    schema: web::Data<Arc<Schema>>,
    pool: web::Data<DbPool>,
    request: web::Json<GraphQLRequest>,
) -> WebResult<HttpResponse> {
    let ctx = Context {
        pool: pool.get_ref().to_owned(),
    };

    let res = web::block(move || {
        let res = request.execute(&schema, &ctx);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(WebError::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql/"))
}

pub fn service() -> Scope {
    let schema = Arc::new(Schema::new(Query, Mutation));
    web::scope("/graphql")
        .data(schema)
        .route("/", web::post().to(graphql))
        .route("/playground", web::get().to(playground))
}
