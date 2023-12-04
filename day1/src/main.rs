use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use anyhow::{Result, anyhow};

const RADIX: u32 = 10;

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("./input")?;
    let maybe_sum: Result<u32> = BufReader::new(file).lines()
        .map(|maybe_line| {
            let line = maybe_line?;
            let first_digit = line
                .chars()
                .find(|c|c.is_digit(RADIX))
                .ok_or(anyhow!("No digit on line {}", line))?
                .to_digit(RADIX)
                .unwrap();
            let second_digit = line
                .chars()
                .rfind(|c|c.is_digit(RADIX))
                .ok_or(anyhow!("No digit on line {}", line))?
                .to_digit(RADIX)
                .unwrap();
            Ok((first_digit * 10) + second_digit)
        })
        .sum();
    println!("{}", maybe_sum?);
    Ok(())
}
