mod universe;

use anyhow::Result;
use itertools::Itertools;
use std::fs::OpenOptions;
use universe::Universe;

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;
    let mut universe = Universe::from_reader(file)?;

    universe.expand();

    println!("part one: {}", part_one(&universe));

    Ok(())
}

fn part_one(universe: &Universe) -> String {
    universe
        .galaxies()
        .combinations(2)
        .map(|pair| pair[0].distance_to(pair[1]))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use dotenv::dotenv;
    use std::env;
    use std::fs::OpenOptions;
    use universe::Universe;

    #[test]
    fn part_one_tests() -> Result<()> {
        dotenv().ok();

        let file = OpenOptions::new().read(true).open("test")?;
        let mut universe = Universe::from_reader(file)?;

        universe.expand();

        assert_eq!(part_one(&universe), env::var("PART_ONE_TEST")?);

        let file = OpenOptions::new().read(true).open("input")?;
        let mut universe = Universe::from_reader(file)?;

        universe.expand();

        assert_eq!(part_one(&universe), env::var("PART_ONE")?);

        Ok(())
    }
}
