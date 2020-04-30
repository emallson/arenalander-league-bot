use crate::mana_parser::parse_mana;
use crate::models::{Deck, League, User, Match, DeckRecord};
use crate::deck_url;
use actix_web::{
    get, web, HttpRequest, HttpResponse, Responder, Result as WebResult, Scope
};
use anyhow::Result;
use askama::Template;
use diesel::prelude::*;
use diesel::PgConnection;
use qstring::QString;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BTreeMap};
use uuid::Uuid;
use super::DbPool;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DisplayType {
    Creature,
    Planeswalker,
    Artifact,
    Enchantment,
    Instant,
    Sorcery,
    Land,
}

fn display_type(types: &str) -> Option<DisplayType> {
    use DisplayType::*;
    if types.contains("Creature") {
        Some(Creature)
    } else {
        match types.split(',').next().unwrap() {
            "Planeswalker" => Some(Planeswalker),
            "Artifact" => Some(Artifact),
            "Enchantment" => Some(Enchantment),
            "Instant" => Some(Instant),
            "Sorcery" => Some(Sorcery),
            "Land" => Some(Land),
            _ => None,
        }
    }
}

impl std::fmt::Display for DisplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
struct DisplayCard {
    count: usize,
    name: String,
    cmc: f64,                  // uncards on arena, ugh
    cost: Option<Vec<String>>, // lands have no mana cost
    types: String,
    uuid: Uuid,
}

impl PartialOrd for DisplayCard {
    fn partial_cmp(&self, other: &DisplayCard) -> Option<Ordering> {
        Some(match self.cmc.partial_cmp(&other.cmc) {
            None => unreachable!(),
            Some(Ordering::Equal) => self.name.cmp(&other.name),
            Some(other_order) => other_order,
        })
    }
}

impl Eq for DisplayCard {}

impl Ord for DisplayCard {
    fn cmp(&self, other: &DisplayCard) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_record(conn: &PgConnection, deck_id: i32) -> Result<DeckRecord> {
    use crate::schema::deck_records::dsl::*;

    deck_records.filter(id.eq(deck_id)).get_result(conn)
        .map_err(|e| e.into())
}

fn get_matches(conn: &PgConnection, deck_id: i32) -> Result<Vec<DisplayMatch>> {
    use crate::schema::matches::dsl::*;
    use crate::schema::users::dsl::{users, id as uid};
    use crate::schema::decks::dsl::{decks, id, owner};

    let confirmed_matches: Vec<(Match, Deck, User)> = matches
        .filter(confirmed.eq(true).and(winning_deck.eq(deck_id).or(losing_deck.eq(deck_id))))
        .inner_join(decks.on(id.ne(deck_id).and(id.eq(winning_deck).or(id.eq(losing_deck)))))
        .inner_join(users.on(uid.eq(owner)))
        .get_results(conn)?;

    Ok(confirmed_matches.into_iter().map(|(match_, opp_deck, opponent)| {
        DisplayMatch {
            match_,
            opp_deck, 
            opponent,
            deck_id
        }
    }).collect())
}

fn get_mw(conn: &PgConnection, deck_ids: Vec<i32>) -> Result<f64> {
    use crate::schema::deck_records::dsl::*;

    let wl: Vec<(i64, i64)> = deck_records
        .select((match_wins, match_losses))
        .filter(id.eq_any(deck_ids))
        .get_results(conn)?;
    
    let num_opp = wl.len();
    Ok(wl.into_iter().map(|(w, l)| w as f64 / (w + l) as f64).sum::<f64>() / num_opp as f64)
}

fn get_deck(
    conn: &PgConnection,
    deck_id: i32,
    input_token: Option<Uuid>,
) -> Result<Option<DeckTemplate>> {
    use crate::schema::decks::dsl::*;
    use crate::schema::leagues::dsl::leagues;
    use crate::schema::users::dsl::users;

    let (deck_, league_, user_): (Deck, League, User) = decks
        .filter(id.eq(deck_id))
        .inner_join(leagues)
        .inner_join(users)
        .get_result(conn)?;

    if deck_.active {
        // we need to check the token
        if let Some(tok) = input_token {
            use crate::schema::deck_view_tokens::dsl::*;
            let tok_count = deck_view_tokens
                .filter(deck.eq(deck_id).and(token.eq(tok)))
                .count()
                .get_result::<i64>(conn)?;

            if tok_count == 0 {
                warn!("Attempt to view active deck {} without token!", deck_.id);
                return Ok(None); // no token in db
            }
        } else {
            warn!("Attempt to view active deck {} without token!", deck_.id);
            return Ok(None); // no token, no access
        }
    }

    // at this point, we know we have access to the deck
    let contents: Vec<(i32, String, f64, Option<String>, String, Uuid)> = {
        use crate::schema::cards::dsl::*;
        use crate::schema::deck_contents::dsl::*;
        deck_contents
            .filter(deck.eq(deck_id))
            .inner_join(cards.on(scryfalloracleid.eq(card)))
            .select((count, name, convertedmanacost, manacost, types, scryfalloracleid))
            .distinct()
            .get_results(conn)?
    };

    let mut cards = contents
        .into_iter()
        .map(|(count, name, cmc, cost, types, oracleid)| {
            (
                display_type(&types).unwrap(),
                DisplayCard {
                    count: count as usize,
                    name,
                    cmc,
                    cost: cost.map(|c| parse_mana(&c).unwrap()),
                    types,
                    uuid: oracleid
                },
            )
        })
        .fold(BTreeMap::new(), |mut map, (displaytype, card)| {
            let entry = map.entry(displaytype).or_insert_with(|| CardSection(Vec::new()));
            entry.push(card);
            map
        });

    for val in cards.values_mut() {
        val.sort();
    }

    // deal with split+adventure cards a BTreeMap maintains its keys in order, which for enums with
    // #[derive(Ord)] is the order in which they are defined. as a result, Creature come first so
    // adventure cards will always get displayed as a creature. Other split cards are a crapshot.
    let mut uuid_map = BTreeSet::new();

    for val in cards.values_mut() {
        val.0.retain(|card| {
            if uuid_map.contains(&card.uuid) {
                false
            } else {
                uuid_map.insert(card.uuid);
                true
            }
        });
    }

    let matches = get_matches(conn, deck_id)?;
    let record = get_record(conn, deck_id)?;
    let meta = MetaInfo {
        match_wins: record.match_wins,
        match_losses: record.match_losses,
        omw: 100.0 * get_mw(conn, matches.iter().map(|m| m.opponent_deck_id()).collect())?
    };

    Ok(Some(DeckTemplate {
        user: user_,
        league: league_,
        sections: cards,
        meta,
        matches,
    }))
}

type CardSections = BTreeMap<DisplayType, CardSection>;

struct CardSection(pub Vec<DisplayCard>);

impl std::ops::Deref for CardSection {
    type Target = Vec<DisplayCard>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CardSection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CardSection {
    fn count(&self) -> usize {
        self.0.iter().map(|c| c.count).sum()
    }
}

struct MetaInfo {
    match_wins: i64,
    match_losses: i64,
    omw: f64,
}

struct DisplayMatch {
    match_: Match,
    deck_id: i32,
    opponent: User,
    opp_deck: Deck,
}

impl DisplayMatch {
    pub fn is_winner(&self) -> bool {
        self.match_.winning_deck == self.deck_id
    }

    pub fn wins(&self) -> i32 {
        if self.is_winner() {
            self.match_.winner_wins
        } else {
            self.match_.loser_wins
        }
    }

    pub fn losses(&self) -> i32 {
        if self.is_winner() {
            self.match_.loser_wins
        } else {
            self.match_.winner_wins
        }
    }

    pub fn opponent(&self) -> &str {
        self.opponent.name.as_str()
    }

    fn opponent_deck_id(&self) -> i32 {
        if self.is_winner() { self.match_.losing_deck } else { self.match_.winning_deck }
    }

    pub fn opp_deck(&self) -> Option<String> {
        if self.opp_deck.active {
            None
        } else {
            Some(deck_url(self.opponent_deck_id(), None))
        }
    }

    pub fn date(&self) -> String {
        format!("{}", self.match_.date.format("%e %B %Y"))
    }
}

#[derive(Template)]
#[template(path = "deck.html")]
struct DeckTemplate {
    user: User,
    league: League,
    sections: CardSections,
    meta: MetaInfo,
    matches: Vec<DisplayMatch>,
}

#[get("/{id}")]
async fn deck(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    id: web::Path<(i32,)>,
) -> WebResult<impl Responder> {
    let qs = QString::from(req.query_string());
    let conn = pool.get().expect("Unable to get DB connection");

    let token = if let Some(tok_str) = qs.get("token") {
        Uuid::parse_str(tok_str).map(Some).unwrap_or(None)
    } else {
        None
    };

    let tpl = get_deck(&conn, id.0, token).map_err(|e| {
        error!("Unable to retrieve deck: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(tpl) = tpl {
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(tpl.render().unwrap()))
    } else {
        // not allowed to see this or doesn't exist
        Ok(HttpResponse::NotFound().finish())
    }
}

pub fn service() -> Scope {
    web::scope("/deck")
        .service(deck)
}