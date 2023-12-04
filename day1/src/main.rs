use anyhow::{anyhow, Result};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek};

const RADIX: u32 = 10;

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

fn main() -> Result<()> {
    let mut reader = BufReader::new(OpenOptions::new().read(true).open("./input")?);
    part_one(&mut reader)?;
    Ok(())
}
