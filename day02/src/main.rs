use anyhow::{anyhow, bail, Result};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::num::NonZeroUsize;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        use Color::*;
        Ok(match value.trim_start() {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            _ => bail!("Invalid color {value}"),
        })
    }
}

#[derive(Debug, Default)]
struct Set {
    red: Option<NonZeroUsize>,
    green: Option<NonZeroUsize>,
    blue: Option<NonZeroUsize>,
}

impl Set {
    fn power(&self) -> usize {
        [self.red, self.green, self.blue]
            .into_iter()
            .flatten()
            .map(|value| value.get())
            .product()
    }
}

impl TryFrom<&str> for Set {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let split = value.trim_start().split(',');
        split
            .map(|field| {
                let mut split = field.trim_start().split(' ');
                let count: usize = split.next().ok_or(anyhow!("Missing count"))?.parse()?;
                let color = Color::try_from(split.next().ok_or(anyhow!("Missing color"))?)?;
                Ok((count, color))
            })
            .try_fold(
                Set::default(),
                |mut acc, values: Result<_, anyhow::Error>| {
                    let (count, color) = values?;
                    match color {
                        Color::Red => acc.red = Some(count.try_into()?),
                        Color::Green => acc.green = Some(count.try_into()?),
                        Color::Blue => acc.blue = Some(count.try_into()?),
                    }
                    Ok(acc)
                },
            )
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self> {
        let mut split = line.split(':');
        let id: usize = split
            .next()
            .ok_or(anyhow!("Invalid format"))?
            .split(' ')
            .nth(1)
            .ok_or(anyhow!("Invalid format"))?
            .parse()?;
        let sets = split
            .next()
            .ok_or(anyhow!("Missing sets"))?
            .split(';')
            .map(Set::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { id, sets })
    }
}

impl Game {
    #[rustfmt::skip] // fmt formats below match badly
    fn is_possible(&self) -> bool {
        let max_set = self.sets.iter().fold(Set::default(), |mut acc, set| {
            acc.red = acc.red.max(set.red);
            acc.green = acc.green.max(set.green);
            acc.blue = acc.blue.max(set.blue);
            acc
        });
        match max_set {
            Set { red: Some(red), .. } if red.get() > MAX_RED => false,
            Set { green: Some(green), .. } if green.get() > MAX_GREEN => false,
            Set { blue: Some(blue), .. } if blue.get() > MAX_BLUE => false,
            _ => true,
        }
    }
}

fn part_one(games: &[Game]) {
    let sum: usize = games
        .iter()
        .filter_map(|game| {
            if game.is_possible() {
                return Some(game.id);
            }
            None
        })
        .sum();
    println!("{sum}");
}

fn part_two(games: &[Game]) {
    let sum: usize = games
        .iter()
        .map(|game| {
            game.sets
                .iter()
                .fold(Set::default(), |mut acc, set| {
                    acc.red = acc.red.max(set.red);
                    acc.green = acc.green.max(set.green);
                    acc.blue = acc.blue.max(set.blue);
                    acc
                })
                .power()
        })
        .sum();
    println!("{sum}");
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("./input")?;
    let games: Vec<_> = BufReader::new(file)
        .lines()
        .map(|maybe_line| Game::try_from(maybe_line?.as_str()))
        .collect::<Result<_>>()?;
    part_one(&games);
    part_two(&games);
    Ok(())
}
