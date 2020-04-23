use nom::character::complete::{space1, char, alphanumeric1, digit1, line_ending, space0, none_of};
use nom::combinator::{map, opt, map_res};
use nom::bytes::complete::tag;
use nom::sequence::{delimited, tuple, preceded, separated_pair, terminated};
use nom::multi::many0;
use nom::IResult;
use anyhow::Result;
use thiserror::Error;
use scryfall::card::Card;

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
    pub set: String,
    pub code: u32,
}

const BASICS: [&str; 5] = ["Plains", "Island", "Swamp", "Mountain", "Forest"];

fn card(input: &str) -> IResult<&str, RawDeckEntry> {
    let count = map_res(digit1, |s: &str| s.parse::<u32>());
    let title = map(many0(none_of("()")), |s| s.into_iter().collect::<String>().trim().to_string());
    let setcode = delimited(char('('), alphanumeric1, char(')'));
    let collector_number = map_res(digit1, |s: &str| s.parse::<u32>());
    map(tuple((count, space1, title, space0, setcode, space1, collector_number)), |t: (u32, _, _, _, _, _, u32)| {
        RawDeckEntry {
            count: t.0,
            name: t.2,
            set: t.4.to_string(),
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
    name: String,
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

fn validate_decklist(list: RawDeck) -> Result<Deck> {
    // Step 1: Check for sideboard errors.
    if let Some(sb) = list.sideboard {
        if sb.len() > 0 {
            return Err(DeckError::NonEmptySideboard(sb.len() as u32).into());
        }
    }

    // Step 2: Check if too many cards are in the list.
    let count = list.main.iter().map(|e| e.count).sum::<u32>();
    if count != 100 {
        return Err(DeckError::WrongSize(count).into());
    }

    // Step 3: Check for invalid card names.
    let cards = list.main.iter().map(|entry| (entry, Card::named(entry.name.as_str()))).collect::<Vec<_>>();
    let invalid = cards.iter().filter(|(_, c)| c.is_err()).collect::<Vec<_>>();

    if !invalid.is_empty() {
        return Err(DeckError::InvalidCard(invalid.into_iter().map(|(e, _)| e.name.clone()).collect::<Vec<_>>().join(", ")).into());
    }

    // Step 4: Check for too many copies of individual cards. This uses the English names from Scryfall
    let cards = cards.into_iter().map(|(e, c)| (e, c.unwrap())).collect::<Vec<_>>();
    for (entry, card) in &cards {
        validate_count(&card.name, entry.count)?;
    }

    // TODO: Step 5: Check points

    Ok(cards.into_iter().map(|(e, c)| {
        NormalizedCardEntry {
            name: c.name,
            count: e.count
        }
    }).collect())
}

pub fn parse_deck(deck: &str) -> Result<Deck> {
    match parse_decklist(deck) {
        Ok((_, deck)) => validate_decklist(deck),
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

    #[test]
    fn parse_card() {
        use super::{RawDeckEntry, card};
        let (_, rest) = card(TEST_CARD).unwrap();
        assert_eq!(rest, RawDeckEntry {
            count: 1,
            name: "Lazotep Plating".to_string(),
            set: "WAR".to_string(),
            code: 59
        });

        let (_, rest) = card(TEST_ISLAND).unwrap();
        assert_eq!(rest, RawDeckEntry {
            count: 12,
            name: "Island".to_string(),
            set: "ANA".to_string(),
            code: 57
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
        use super::{parse_decklist, validate_decklist};
        let (_, deck) = parse_decklist(TEST_LIST).unwrap();
        validate_decklist(deck).unwrap();

        let (_, deck) = parse_decklist(TEST_LIST_INVALID).unwrap();
        validate_decklist(deck).unwrap_err();
    }
}