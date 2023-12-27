mod universe;

use anyhow::Result;
use std::fs::OpenOptions;
use universe::Universe;

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;
    let universe = Universe::from_reader(file)?;

    println!("part one: {}", part_one(&universe));
    println!("part two: {}", part_two(&universe));

    Ok(())
}

fn part_one(universe: &Universe) -> String {
    universe.distances(2).sum::<usize>().to_string()
}

fn part_two(universe: &Universe) -> String {
    universe.distances(1_000_000).sum::<usize>().to_string()
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
        let universe = Universe::from_reader(file)?;

        assert_eq!(part_one(&universe), env::var("PART_ONE_TEST")?);

        let file = OpenOptions::new().read(true).open("input")?;
        let universe = Universe::from_reader(file)?;

        assert_eq!(part_one(&universe), env::var("PART_ONE")?);

        Ok(())
    }

    #[test]
    fn part_two_tests() -> Result<()> {
        dotenv().ok();

        let file = OpenOptions::new().read(true).open("input")?;
        let universe = Universe::from_reader(file)?;

        assert_eq!(part_two(&universe), env::var("PART_TWO")?);

        Ok(())
    }
}
