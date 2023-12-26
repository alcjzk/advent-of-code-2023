use anyhow::{anyhow, Result};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek};

const RADIX: u32 = 10;
const DIGITS: &[&str; 9] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part_one<R: BufRead + Seek>(reader: &mut R) -> Result<()> {
    reader.rewind()?;
    let maybe_sum: Result<u32> = reader
        .lines()
        .map(|maybe_line| {
            let line = maybe_line?;
            let first_digit = line
                .chars()
                .find(|c| c.is_digit(RADIX))
                .ok_or(anyhow!("No digit on line {}", line))?
                .to_digit(RADIX)
                .unwrap();
            let second_digit = line
                .chars()
                .rfind(|c| c.is_digit(RADIX))
                .ok_or(anyhow!("No digit on line {}", line))?
                .to_digit(RADIX)
                .unwrap();
            Ok((first_digit * 10) + second_digit)
        })
        .sum();
    println!("{}", maybe_sum?);
    Ok(())
}

fn first_digit(line: &str) -> Option<u32> {
    let maybe_digit = line
        .find(|c: char| c.is_digit(RADIX))
        .map(|pos| (pos, line.chars().nth(pos).unwrap().to_digit(RADIX).unwrap()));

    let maybe_digit_string = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(i, string)| Some((line.find(string)?, i as u32 + 1)))
        .min_by_key(|(pos, _)| *pos);

    let (_, digit) = [maybe_digit, maybe_digit_string]
        .into_iter()
        .flatten()
        .min_by_key(|(pos, _)| *pos)?;
    Some(digit)
}

fn second_digit(line: &str) -> Option<u32> {
    let maybe_digit = line
        .rfind(|c: char| c.is_digit(RADIX))
        .map(|pos| (pos, line.chars().nth(pos).unwrap().to_digit(RADIX).unwrap()));

    let maybe_digit_string = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(i, string)| Some((line.rfind(string)?, i as u32 + 1)))
        .max_by_key(|(pos, _)| *pos);

    let (_, digit) = [maybe_digit, maybe_digit_string]
        .into_iter()
        .flatten()
        .max_by_key(|(pos, _)| *pos)?;
    Some(digit)
}

fn part_two<R: BufRead + Seek>(reader: &mut R) -> Result<()> {
    reader.rewind()?;
    let maybe_sum: Result<u32> = reader
        .lines()
        .map(|maybe_line| {
            let line = maybe_line?;
            let first_digit = first_digit(&line).ok_or(anyhow!("No digit on line {}", line))?;
            let second_digit = second_digit(&line).ok_or(anyhow!("No digit on line {}", line))?;
            Ok((first_digit * 10) + second_digit)
        })
        .sum();
    println!("{}", maybe_sum?);
    Ok(())
}

fn main() -> Result<()> {
    let mut reader = BufReader::new(OpenOptions::new().read(true).open("./input")?);
    part_one(&mut reader)?;
    part_two(&mut reader)?;

    Ok(())
}
