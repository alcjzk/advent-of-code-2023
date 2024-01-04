use anyhow::{anyhow, bail, Error, Result};
use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]

struct PartTwo;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct PartOne;

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
#[repr(u8)]
enum Card<T> {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
    _Unreachable(Infallible, PhantomData<T>),
}

impl<T> Card<T> {
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Ord for Card<PartOne> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.discriminant().cmp(&other.discriminant())
    }
}

impl Ord for Card<PartTwo> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (Card::J, Card::J) => Ordering::Equal,
            (Card::J, _) => Ordering::Less,
            (_, Card::J) => Ordering::Greater,
            _ => self.discriminant().cmp(&other.discriminant()),
        }
    }
}

impl<T> TryFrom<char> for Card<T> {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        use Card::*;

        Ok(match value {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => bail!("Unknown card '{value}'"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<[u8; 2]> for HandKind {
    fn from(value: [u8; 2]) -> Self {
        use HandKind::*;
        match value {
            [_, 5] => FiveOfKind,
            [_, 4] => FourOfKind,
            [2, 3] => FullHouse,
            [_, 3] => ThreeOfKind,
            [2, 2] => TwoPair,
            [_, 2] => OnePair,
            [_, _] => HighCard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand<T>([Card<T>; 5]);

trait HandExt {
    fn kind(&self) -> HandKind;
}

impl HandExt for Hand<PartOne> {
    fn kind(&self) -> HandKind {
        let mut dup = self.0;
        dup.sort_unstable();

        let mut previous = None;
        let mut matching_kind_counts: [u8; 2] = [1, 1];
        let mut kind_changed = false;

        for card in dup {
            match previous {
                Some(prev) if prev == card => match kind_changed {
                    false => matching_kind_counts[0] += 1,
                    true => matching_kind_counts[1] += 1,
                },
                Some(_) if matching_kind_counts[0] > 1 => {
                    kind_changed = true;
                    previous = Some(card);
                }
                _ => previous = Some(card),
            }
        }

        matching_kind_counts.sort_unstable();
        HandKind::from(matching_kind_counts)
    }
}

impl HandExt for Hand<PartTwo> {
    fn kind(&self) -> HandKind {
        use HandKind::*;

        let mut dup = self.0;
        dup.sort_unstable();

        let mut previous = None;
        let mut matching_kind_counts: [u8; 2] = [1, 1];
        let mut kind_changed = false;
        let mut jokers = 0;

        for card in dup {
            if card == Card::J {
                jokers += 1;
                previous = Some(card);
                continue;
            }
            match previous {
                Some(prev) if prev == card => match kind_changed {
                    false => matching_kind_counts[0] += 1,
                    true => matching_kind_counts[1] += 1,
                },
                Some(_) if matching_kind_counts[0] > 1 => {
                    kind_changed = true;
                    previous = Some(card);
                }
                _ => previous = Some(card),
            }
        }

        matching_kind_counts.sort_unstable();
        match jokers {
            5 => return FiveOfKind,
            _ => matching_kind_counts[1] += jokers,
        }

        HandKind::from(matching_kind_counts)
    }
}

impl<T> Ord for Hand<T>
where
    Hand<T>: HandExt,
    Card<T>: Ord,
    T: Ord + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_ordering = self.kind().cmp(&other.kind());
        if kind_ordering != Ordering::Equal {
            return kind_ordering;
        }
        self.0
            .iter()
            .copied()
            .zip(other.0.iter().copied())
            .find(|(card, other)| *card != *other)
            .map_or(Ordering::Equal, |(card, other)| card.cmp(&other))
    }
}

impl<T> PartialOrd for Hand<T>
where
    Hand<T>: HandExt,
    Card<T>: Ord,
    T: Ord + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> TryFrom<&str> for Hand<T> {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut chars = value.chars();
        Ok(Hand([
            chars
                .next()
                .ok_or(anyhow!("Invalid hand format"))?
                .try_into()?,
            chars
                .next()
                .ok_or(anyhow!("Invalid hand format"))?
                .try_into()?,
            chars
                .next()
                .ok_or(anyhow!("Invalid hand format"))?
                .try_into()?,
            chars
                .next()
                .ok_or(anyhow!("Invalid hand format"))?
                .try_into()?,
            chars
                .next()
                .ok_or(anyhow!("Invalid hand format"))?
                .try_into()?,
        ]))
    }
}

type Bid = usize;

#[derive(Debug)]
struct Game<T> {
    hands: Vec<(Hand<T>, Bid)>,
}

impl<T> Game<T>
where
    T: Clone + Ord + fmt::Debug,
    Hand<T>: Ord + HandExt,
{
    fn new(hands: Vec<(Hand<T>, Bid)>) -> Self {
        Self { hands }
    }
    fn winnings(&self) -> usize {
        let mut dup = self.hands.clone();
        dup.sort_by(|(hand, _), (other, _)| hand.cmp(other));
        dup.iter()
            .enumerate()
            .rev()
            .map(|(index, (_, bid))| (index + 1) * bid)
            .sum()
    }
}

impl<T> FromIterator<(Hand<T>, Bid)> for Game<T>
where
    T: Clone + Ord + fmt::Debug,
    Hand<T>: Ord + HandExt,
{
    fn from_iter<I: IntoIterator<Item = (Hand<T>, Bid)>>(iter: I) -> Self {
        Game::new(iter.into_iter().collect())
    }
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;

    let game: Game<PartOne> = BufReader::new(file)
        .lines()
        .map(|maybe_line| {
            let line = maybe_line?;
            let mut split = line.split_ascii_whitespace();
            let hand: Hand<_> = split
                .next()
                .ok_or(anyhow!("Missing hand on line '{line}'"))?
                .try_into()?;
            let bid: Bid = split
                .next()
                .ok_or(anyhow!("Missing bid on line '{line}'"))?
                .parse::<Bid>()?;
            Ok((hand, bid))
        })
        .collect::<Result<_>>()?;

    println!("{}", game.winnings());

    // SAFETY: Because `PartOne` and `PartTwo` are zero-sized, and only used as implementation
    // markers, this transmute should not cause issues.
    let game: Game<PartTwo> = unsafe { std::mem::transmute(game) };

    println!("{}", game.winnings());

    Ok(())
}
