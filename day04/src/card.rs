use anyhow::{anyhow, Error, Result};
use std::cell::Cell;

#[derive(Debug)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
    won_cards_count: Cell<Option<usize>>,
}

impl Card {
    pub fn matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
    pub fn points(&self) -> usize {
        let matches = self.matches();
        if matches > 0 {
            return 2usize.pow((matches - 1) as u32);
        }
        0
    }
    pub fn won_cards<'a>(&self, cards: &'a [Card]) -> Vec<&'a Card> {
        let matches = self.matches();

        for count in (1..=matches).rev() {
            let start = self.id + 1;
            let end = start + count;
            if let Some(won_cards) = cards.get(start..end) {
                return won_cards.iter().collect();
            }
        }
        Vec::new()
    }
    pub fn cards_worth(&self, cards: &[Card]) -> usize {
        if let Some(count) = self.won_cards_count.get() {
            return count;
        }
        let won_cards = self.won_cards(cards);
        let count = match won_cards.len() {
            0 => 1,
            _ => {
                won_cards
                    .iter()
                    .map(|card| card.cards_worth(cards))
                    .sum::<usize>()
                    + 1
            }
        };
        self.won_cards_count.set(Some(count));
        count
    }
}

impl TryFrom<(usize, String)> for Card {
    type Error = Error;

    fn try_from(value: (usize, String)) -> Result<Self> {
        fn _numbers(string: &str) -> Result<Vec<usize>> {
            string
                .split_ascii_whitespace()
                .map(|number| Ok(number.parse::<usize>()?))
                .collect::<Result<_>>()
        }

        let (id, line) = value;
        let values = line
            .split(':')
            .nth(1)
            .ok_or(anyhow!("Invalid card format '{line}'"))?;

        let mut split = values.split('|');
        let winning_numbers = _numbers(
            split
                .next()
                .ok_or(anyhow!("Invalid card format '{line}'"))?,
        )?;
        let numbers = _numbers(
            split
                .next()
                .ok_or(anyhow!("Invalid card format '{line}'"))?,
        )?;

        Ok(Card {
            id,
            winning_numbers,
            numbers,
            won_cards_count: Cell::default(),
        })
    }
}
