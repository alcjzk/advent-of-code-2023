mod pattern;

use anyhow::Result;
use pattern::Patterns;
use std::fs::OpenOptions;

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;
    let patterns = Patterns::read(file)?;
    println!("part one: {}", patterns.summarize());
    println!("part two: {}", patterns.summarize2());
    Ok(())
}
