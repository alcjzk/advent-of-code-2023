use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use std::fs::OpenOptions;
use std::{
    io::{BufRead, BufReader},
    ops::Range,
};

#[derive(Debug)]
struct Map(Vec<Mapping>);

impl Map {
    fn map(&self, value: usize) -> usize {
        self.0
            .iter()
            .find_map(|mapping| mapping.map(value))
            .unwrap_or(value)
    }
}

impl FromIterator<Mapping> for Map {
    fn from_iter<T: IntoIterator<Item = Mapping>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Debug)]
struct Mapping {
    source_range: Range<usize>,
    difference: isize,
}

impl TryFrom<String> for Mapping {
    type Error = Error;

    fn try_from(line: String) -> Result<Self> {
        let mut split = line.split_ascii_whitespace();
        let destination_start = split
            .next()
            .ok_or(anyhow!("Missing value on line '{line}'"))?
            .parse::<usize>()?;
        let source_start = split
            .next()
            .ok_or(anyhow!("Missing value on line '{line}'"))?
            .parse::<usize>()?;
        let length = split
            .next()
            .ok_or(anyhow!("Missing value on line '{line}'"))?
            .parse::<usize>()?;
        Ok(Mapping::new(source_start, destination_start, length))
    }
}

impl Mapping {
    fn new(source_start: usize, destination_start: usize, length: usize) -> Self {
        Self {
            source_range: Range {
                start: source_start,
                end: source_start + length,
            },
            difference: (destination_start as isize) - (source_start as isize),
        }
    }
    fn map(&self, value: usize) -> Option<usize> {
        if self.source_range.contains(&value) {
            return Some(((value as isize) + self.difference) as usize);
        }
        None
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn new(mut lines: impl Iterator<Item = String>) -> Result<Self> {
        fn _map_from(lines: impl Iterator<Item = String>) -> Result<Map> {
            lines
                .map_while(|line| {
                    if line.is_empty() {
                        return None;
                    }
                    Some(Mapping::try_from(line))
                })
                .collect::<Result<Map>>()
        }
        let seed_line = lines.next().ok_or(anyhow!("Missing seed line"))?;
        let seeds = seed_line
            .strip_prefix("seeds:")
            .ok_or(anyhow!("Invalid seed line"))?
            .split_ascii_whitespace()
            .map(|seed| seed.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        let _ = lines.nth(1).ok_or(anyhow!("Unexpected EOF"))?;
        let seed_to_soil: Map = _map_from(&mut lines)?;
        let _ = lines.nth(0).ok_or(anyhow!("Unexpected EOF"))?;
        let soil_to_fertilizer = _map_from(&mut lines)?;
        let _ = lines.nth(0).ok_or(anyhow!("Unexpected EOF"))?;
        let fertilizer_to_water = _map_from(&mut lines)?;
        let _ = lines.nth(0).ok_or(anyhow!("Unexpected EOF"))?;
        let water_to_light = _map_from(&mut lines)?;
        let _ = lines.nth(0).ok_or(anyhow!("Unexpected EOF"))?;
        let light_to_temperature = _map_from(&mut lines)?;
        let _ = lines.nth(0).ok_or(anyhow!("Unexpected EOF"))?;
        let temperature_to_humidity = _map_from(&mut lines)?;
        let _ = lines.nth(0).ok_or(anyhow!("Unexpected EOF"))?;
        let humidity_to_location = _map_from(&mut lines)?;
        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
    fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        self.humidity_to_location.map(humidity)
    }
    fn seeds_as_ranges(&self) -> Vec<Range<usize>> {
        self.seeds
            .iter()
            .copied()
            .tuples::<(usize, usize)>()
            .map(|(start, length)| Range {
                start,
                end: start + length,
            })
            .collect()
    }
}

fn part_one(almanac: &Almanac) {
    let location = almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap();

    println!("{location}");
}

fn part_two(almanac: &Almanac) {
    let location = almanac
        .seeds_as_ranges()
        .into_iter()
        .flat_map(|range| range.into_iter())
        .map(|seed| almanac.seed_to_location(seed))
        .min()
        .unwrap();

    println!("{location}");
}
fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("./input")?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    let almanac = Almanac::new(lines.into_iter())?;

    part_one(&almanac);
    part_two(&almanac);

    Ok(())
}
