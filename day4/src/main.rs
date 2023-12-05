mod card;

use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use anyhow::Result;
use card::Card;

fn part_one(cards: &[Card]) {
    let sum: usize = cards.iter()
        .map(|card|card.points())
        .sum();

    println!("{sum}");
}

fn won_cards<'a, T>(cards: T, original: &'a [Card]) -> Vec<&'a Card>
where
    T: IntoIterator<Item = &'a Card> + 'a
{
        cards.into_iter()
            .map(|card| card.won_cards(original))
            .flatten()
            .collect()
}

fn part_two(cards: &[Card]) {
    let mut count = cards.len();

    let mut next: Vec<&Card> = won_cards(cards, cards);
    while !next.is_empty() {
        count += next.len();
        next = won_cards(next, cards);
    }

    println!("{count}");
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("./input")?;

    let cards = BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(id, maybe_line)| Ok(Card::try_from((id, maybe_line?))?))
        .collect::<Result<Vec<_>>>()?;

    part_one(&cards);
    part_two(&cards);

    Ok(())
}
