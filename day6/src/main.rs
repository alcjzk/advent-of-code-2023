#![allow(unused)]
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use anyhow::{Result, anyhow};

#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn record_beat_ways_count(&self) -> usize {
        let hold_time = self.time / 2;
        (self.hold_time_lowest()..=self.hold_time_highest()).count()
    }
    fn hold_time_lowest(&self) -> usize {
        let mut hold_time = 0;
        while (distance(hold_time, self.time) <= self.record_distance) {
            hold_time += 1;
        }
        hold_time
    }
    fn hold_time_highest(&self) -> usize {
        let mut hold_time = self.time;
        while (distance(hold_time, self.time) <= self.record_distance) {
            hold_time -= 1;
        }
        hold_time
    }
}

fn distance(hold_time: usize, time: usize) -> usize {
    hold_time * (time - hold_time)
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("input")?;
    let mut lines = BufReader::new(file).lines();
    let times: Vec<usize> = lines.next()
        .ok_or(anyhow!("Missing times in input"))??
        .strip_prefix("Time:")
        .ok_or(anyhow!("Invalid input format"))?
        .split_ascii_whitespace()
        .map(|value|value.parse::<usize>())
        .collect::<Result<_, _>>()?;
    let distances: Vec<usize> = lines.next()
        .ok_or(anyhow!("Missing distances in input"))??
        .strip_prefix("Distance:")
        .ok_or(anyhow!("Invalid input format"))?
        .split_ascii_whitespace()
        .map(|value|value.parse::<usize>())
        .collect::<Result<_, _>>()?;
    let races: Vec<Race> = times.into_iter()
        .zip(distances.into_iter())
        .map(|(time, record_distance)| Race {time, record_distance})
        .collect();
    let product: usize = races.iter().map(|race|race.record_beat_ways_count()).product();
    println!("{product}");
    Ok(())
}
