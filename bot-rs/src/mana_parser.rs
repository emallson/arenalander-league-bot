use nom::character::complete::none_of;
use nom::combinator::{map, complete};
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::multi::many0;
use nom::IResult;
use anyhow::{anyhow, Result};

fn parse_internal(input: &str) -> IResult<&str, Vec<String>> {
    let raw_symbol = delimited(tag("{"), many0(none_of("{}")), tag("}"));
    let symbol = map(raw_symbol, |val| val.into_iter().collect::<String>().replace("/", "").to_lowercase());
    
    let all_symbols = complete(many0(symbol));
        
    all_symbols(input)
}

pub fn parse_mana(input: &str) -> Result<Vec<String>> {
    match parse_internal(input) {
        Ok((_, val)) => Ok(val),
        e => Err(anyhow!("Error parsing mana cost {:?}: {:?}", input, e))
    }
}