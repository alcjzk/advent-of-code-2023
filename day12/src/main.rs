mod input;

use anyhow::Result;
use input::Input;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;

    let mut part_one_answer = 0;
    let mut part_two_answer = 0;

    for maybe_line in BufReader::new(file).lines() {
        let input = Input::new(&maybe_line?)?;
        part_one_answer += input.arrangement_count();
        part_two_answer += input.unfold().arrangement_count();
    }

    println!("part one: {part_one_answer}");
    println!("part two: {part_two_answer}");

    Ok(())
}
