mod card;

use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use anyhow::Result;
use card::Card;

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("./input")?;

    let cards = BufReader::new(file)
        .lines()
        .map(|maybe_line| Ok(Card::try_from(maybe_line?)?))
        .collect::<Result<Vec<_>>>()?;

    let sum: usize = cards.iter().map(|card|card.points()).sum();

    println!("{sum}");
    Ok(())
}
