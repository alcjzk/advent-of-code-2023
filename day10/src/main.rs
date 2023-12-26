mod map;

use anyhow::Result;
use map::Map;
use std::fs::OpenOptions;

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;
    let mut map = Map::read(file)?;

    map.sanitize();

    println!("part one: {}", map.path().count() / 2);
    println!("part two: {}", map.enclosed_tiles_count()?);

    Ok(())
}
