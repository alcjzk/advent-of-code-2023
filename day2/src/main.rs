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

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("./input")?;
    let sum: usize = BufReader::new(file)
        .lines()
        .filter_map(
            |maybe_line| match Game::try_from(maybe_line.unwrap().as_str()) {
                Ok(game) if game.is_possible() => Some(game.id),
                Ok(_) => None,
                Err(error) => {
                    eprint!("{error:?}");
                    None
                }
            },
        )
        .sum();
    println!("{sum}");
    Ok(())
}
