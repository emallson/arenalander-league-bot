use nom::character::complete::{space1, char, alphanumeric1, digit1, line_ending, space0, none_of};
use nom::combinator::{map, opt, map_res};
use nom::bytes::complete::tag;
use nom::sequence::{delimited, tuple, preceded, separated_pair, terminated};
use nom::multi::many0;
use nom::IResult;
use anyhow::Result;
use thiserror::Error;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum DeckError {
    #[error("Unable to parse decklist ({0})")]
    ParseError(String),
    #[error("Wrong size: {0} (expected 100")]
    WrongSize(u32),
    #[error("Too many copies of '{name}' ({count}, maximum {max})")]
    InvalidCount {
        name: String,
        count: u32,
        max: u32,
    },
    #[error("Invalid cards: {0}")]
    InvalidCard(String),
    #[error("Too many points: {points} (maximum is 10). Pointed Cards: {cards}")]
    TooManyPoints {
        points: u32,
        cards: String,
    },
    #[error("Sideboards are not allowed (you have {0} sideboard cards)")]
    NonEmptySideboard(u32)
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawDeckEntry {
    pub name: String,
    pub count: u32,
    pub set: Option<String>,
    pub code: Option<u32>,
}

const BASICS: [&str; 5] = ["Plains", "Island", "Swamp", "Mountain", "Forest"];

fn card(input: &str) -> IResult<&str, RawDeckEntry> {
    let count = map_res(digit1, |s: &str| s.parse::<u32>());
    let title = map(many0(none_of("()")), |s| s.into_iter().collect::<String>().trim().to_string());
    let setcode = map(delimited(char('('), alphanumeric1, char(')')), |code| if code == "DAR" { "DOM" } else { code });
    let collector_number = map_res(digit1, |s: &str| s.parse::<u32>());
    map(tuple((count, space1, title, space0, opt(setcode), space0, opt(collector_number))), |t: (u32, _, _, _, _, _, _)| {
        RawDeckEntry {
            count: t.0,
            name: t.2,
            set: t.4.map(|s| s.to_string()),
            code: t.6
        }
    })(input)
}

fn parse_decklist(list: &str) -> IResult<&str, RawDeck> {
    let deck = preceded(tuple((tag("Deck"), line_ending)), many0(terminated(card, opt(line_ending))));
    let sideboard = preceded(tuple((tag("Sideboard"), line_ending)), many0(terminated(card, opt(line_ending))));
    let decklist = map(separated_pair(deck, opt(many0(line_ending)), opt(sideboard)), |(main, side)| {
        RawDeck {
            main, sideboard: side
        }
    });

    decklist(list)
}

#[derive(Debug)]
struct RawDeck {
    main: Vec<RawDeckEntry>,
    sideboard: Option<Vec<RawDeckEntry>>
}

#[derive(Debug)]
pub struct NormalizedCardEntry {
    id: u32,
    count: u32,
}

pub type Deck = Vec<NormalizedCardEntry>;

fn validate_count(name: &str, count: u32) -> Result<()> {
    if BASICS.contains(&name) || name == "Rat Colony" || name == "Persistent Petitioners" {
        Ok(())
    } else if name == "Seven Dwarves"  {
        if count <= 7 {
            Ok(())
        } else {
            Err(DeckError::InvalidCount {
                name: name.to_string(),
                count,
                max: 7
            }.into())
        }
    } else if count > 1 {
        Err(DeckError::InvalidCount {
            name: name.to_string(),
            count,
            max: 1
        }.into())
    } else {
        Ok(())
    }
}

// TODO: bulk lookup
fn lookup_card(conn: &PgConnection, card: &RawDeckEntry) -> Option<u32> {
    use super::cards::cards::dsl::*;

    if card.code.is_some() && card.set.is_some() {
        // prefer lookup by setcode + number
        cards.select(id).filter(setcode.eq(card.set.as_ref().unwrap()).and(number.eq(card.code.unwrap().to_string())))
            .first(conn).optional()
            .expect("Unable to connect to DB for card lookup.").map(|u: i64| u as u32)
    } else {
        cards.select(id).filter(name.eq(&card.name))
            .first(conn).optional()
            .expect("Unable to connect to DB for card lookup.").map(|u: i64| u as u32)
    }
}

fn validate_decklist(conn: &PgConnection, list: RawDeck) -> Result<Deck> {
    // Step 1: Check for sideboard errors.
    if let Some(sb) = list.sideboard {
        if !sb.is_empty() {
            return Err(DeckError::NonEmptySideboard(sb.len() as u32).into());
        }
    }

    // Step 2: Check if too many cards are in the list.
    let count = list.main.iter().map(|e| e.count).sum::<u32>();
    if count != 100 {
        return Err(DeckError::WrongSize(count).into());
    }

    // Step 3: Check for invalid card names.
    let cards = list.main.iter().map(|entry| (entry, lookup_card(conn, entry))).collect::<Vec<_>>();
    let invalid = cards.iter().filter(|(_, c)| c.is_none()).collect::<Vec<_>>();

    if !invalid.is_empty() {
        return Err(DeckError::InvalidCard(invalid.into_iter().map(|(e, _)| e.name.clone()).collect::<Vec<_>>().join(", ")).into());
    }

    // Step 4: Check for too many copies of individual cards.
    let cards = cards.into_iter().map(|(e, c)| (e, c.unwrap())).collect::<Vec<_>>();
    let mut counts = HashMap::new();
    // merge the counts so you can't do e.g. 1 a 1 b 1 a and get away with it
    for (entry, _card) in &cards {
        *counts.entry(&entry.name).or_insert(0) += entry.count;
    }
    for (name, count) in &counts {
        validate_count(name, *count)?;
    }

    // TODO: Step 5: Check points

    Ok(cards.into_iter().map(|(e, c)| {
        NormalizedCardEntry {
            id: c,
            count: e.count
        }
    }).collect())
}

pub fn parse_deck(conn: &PgConnection, deck: &str) -> Result<Deck> {
    match parse_decklist(deck) {
        Ok((_, deck)) => validate_decklist(conn, deck),
        Err(e) => {
            println!("{:?}", e);
            Err(DeckError::ParseError(format!("{}", e)).into())
        }
    }
}

#[cfg(test)]
mod test {
    const TEST_LIST: &str = include_str!("test_uw.txt");
    const TEST_LIST_INVALID: &str = include_str!("test_invalid.txt");

    const TEST_CARD: &str = "1 Lazotep Plating (WAR) 59";
    const TEST_ISLAND: &str = "12 Island (ANA) 57";
    const TEST_PARTIAL: &str = "47 Mountain";

    #[test]
    fn parse_card() {
        use super::{RawDeckEntry, card};
        let (_, rest) = card(TEST_CARD).unwrap();
        assert_eq!(rest, RawDeckEntry {
            count: 1,
            name: "Lazotep Plating".to_string(),
            set: Some("WAR".to_string()),
            code: Some(59)
        });

        let (_, rest) = card(TEST_ISLAND).unwrap();
        assert_eq!(rest, RawDeckEntry {
            count: 12,
            name: "Island".to_string(),
            set: Some("ANA".to_string()),
            code: Some(57)
        });

        let (_, rest) = card(TEST_PARTIAL).unwrap();
        assert_eq!(rest, RawDeckEntry {
            count: 47,
            name: "Mountain".to_string(),
            set: None,
            code: None
        });
    }

    #[test]
    fn parse_uw_fliers() {
        use super::parse_decklist;
        println!("{}", TEST_LIST);
        let (_, deck) = parse_decklist(TEST_LIST).unwrap();
        assert_eq!(deck.main.len(), 79);
        assert_eq!(deck.sideboard, None);
    }

    #[test]
    fn validation() {
        use std::env;
        use diesel::pg::PgConnection;
        use diesel::prelude::*;

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");
        let conn = PgConnection::establish(&database_url)
            .expect("Unable to connect to database.");
        use super::{parse_decklist, validate_decklist};
        let (_, deck) = parse_decklist(TEST_LIST).unwrap();
        validate_decklist(&conn, deck).unwrap();

        let (_, deck) = parse_decklist(TEST_LIST_INVALID).unwrap();
        validate_decklist(&conn, deck).unwrap_err();
    }
}