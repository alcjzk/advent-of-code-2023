use anyhow::{anyhow, bail, Result};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

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
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn power(&self) -> usize {
        self.red.max(1) * self.green.max(1) * self.blue.max(1)
    }
    fn one() -> Self {
        Self {
            red: 1,
            green: 1,
            blue: 1,
        }
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
                        Color::Red => acc.red = count,
                        Color::Green => acc.green = count,
                        Color::Blue => acc.blue = count,
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
            .map(|field| Set::try_from(field))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { id, sets })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        let max = self.sets.iter().fold(Set::default(), |mut acc, set| {
            acc.red = acc.red.max(set.red);
            acc.green = acc.green.max(set.green);
            acc.blue = acc.blue.max(set.blue);
            acc
        });
        if max.red <= MAX_RED && max.green <= MAX_GREEN && max.blue <= MAX_BLUE {
            return true;
        }
        false
    }
}

fn part_one(games: &Vec<Game>) {
    let sum: usize = games.iter()
        .filter_map(|game|{
            if game.is_possible() {
                return Some(game.id);
            }
            None
        })
        .sum();
    println!("{sum}");
}

fn part_two(games: &Vec<Game>)  {
    let sum: usize = games.iter()
        .map(|game| {
            game.sets.iter()
                .fold(Set::one(), |mut acc, set| {
                    if set.red > 0 {
                        acc.red = acc.red.max(set.red);
                    }
                    if set.green > 0 {
                        acc.green = acc.green.max(set.green);
                    }
                    if set.blue > 0 {
                        acc.blue = acc.blue.max(set.blue);
                    }
                    acc
                }).power()
        }).sum();
    println!("{sum}");
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("./input")?;
    let games: Vec<_> = BufReader::new(file)
        .lines()
        .map(|maybe_line|Game::try_from(maybe_line?.as_str()))
        .collect::<Result<_>>()?;
    part_one(&games);
    part_two(&games);
    Ok(())
}
