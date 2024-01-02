mod node;
mod platform;

use anyhow::Result;
use platform::Platform;
use std::fs::OpenOptions;

const LOOP_SEARCH_OFFSET: usize = 1_000;
const SPIN_ITERATION_TARGET: usize = 1_000_000_000;

fn part_one(platform: Platform) {
    let load = platform.tilt_north().load();

    println!("part one: {load}");
}

fn part_two(platform: Platform) {
    let loop_size = loop_size(platform.clone());
    let offset = spins_until_repeating(platform.clone(), loop_size);
    let load = platform
        .spin_n(offset + (SPIN_ITERATION_TARGET - offset) % loop_size)
        .load();

    println!("part two: {load}");
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;
    let platform = Platform::from_reader(file)?;

    part_one(platform.clone());
    part_two(platform);

    Ok(())
}

fn loop_size(platform: Platform) -> usize {
    let platform = platform.spin_n(LOOP_SEARCH_OFFSET);

    let mut count = 1;
    let mut other = platform.clone().spin();
    while platform != other {
        count += 1;
        other = other.clone().spin();
        assert!(count < 10_000);
    }
    count
}

fn spins_until_repeating(mut platform: Platform, loop_size: usize) -> usize {
    let mut count = 0;

    while platform.clone().spin_n(loop_size) != platform {
        count += 1;
        platform = platform.spin();
        assert!(count < 10_000);
    }
    count
}
