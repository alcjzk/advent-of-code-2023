mod card;

use anyhow::Result;
use card::Card;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

fn part_one(cards: &[Card]) {
    let sum: usize = cards.iter().map(|card| card.points()).sum();

    println!("{sum}");
}

fn part_two(cards: &[Card]) {
    let count = cards
        .iter()
        .map(|card| card.cards_worth(cards))
        .sum::<usize>();

    println!("{count}");
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("./input")?;

    let cards = BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(id, maybe_line)| Card::try_from((id, maybe_line?)))
        .collect::<Result<Vec<_>>>()?;

    part_one(&cards);
    part_two(&cards);

    Ok(())
}
