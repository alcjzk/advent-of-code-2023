use anyhow::{anyhow, bail, Error, Result};
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

use Direction::*;
use Pipe::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Map {
    inner: Vec<Vec<Tile>>,
    start: Position,
    max_x: usize,
    max_y: usize,
    height: usize,
    width: usize,
    sanitized: bool,
}

impl Map {
    pub fn read<R: Read>(reader: R) -> Result<Self> {
        let map = BufReader::new(reader)
            .lines()
            .map(|maybe_line| {
                maybe_line?
                    .chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        // Assert all rows have same length
        debug_assert!(map.iter().tuple_windows().all(|(a, b)| a.len() == b.len()));

        // Assert exacly one starting point in map
        debug_assert!(
            map.iter()
                .flatten()
                .filter(|tile| **tile == Tile::Start)
                .count()
                == 1
        );

        let start = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, tile)| match tile {
                    Tile::Start => Some(Position { x, y }),
                    _ => None,
                })
            })
            .ok_or(anyhow!("Missing starting position in map"))?;

        let height = map.len();
        let width = map[0].len();

        let max_y = height
            .checked_sub(1)
            .ok_or(anyhow!("Invalid map height '0'"))?;
        let max_x = width
            .checked_sub(1)
            .ok_or(anyhow!("Invalid map width '0'"))?;

        let mut map = Self {
            inner: map,
            start,
            max_x,
            max_y,
            height,
            width,
            sanitized: false,
        };

        let mut connected_directions = Vec::with_capacity(2);

        for direction in Direction::all() {
            if let Some(Tile::Pipe(pipe)) = map
                .to_direction_of(start, direction)
                .map(|position| map.get(position))
            {
                if pipe.directions().contains(&direction.opposite()) {
                    connected_directions.push(direction)
                }
            }
        }

        map.inner[start.y][start.x] = Tile::Pipe(Pipe::try_from(connected_directions.as_slice())?);

        Ok(map)
    }

    fn to_direction_of(&self, position: Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => self.north_of(position),
            Direction::South => self.south_of(position),
            Direction::East => self.east_of(position),
            Direction::West => self.west_of(position),
        }
    }

    fn north_of(&self, mut position: Position) -> Option<Position> {
        position.y = position.y.checked_sub(1)?;
        Some(position)
    }

    fn south_of(&self, mut position: Position) -> Option<Position> {
        if position.y >= self.max_y {
            return None;
        }
        position.y += 1;
        Some(position)
    }

    fn west_of(&self, mut position: Position) -> Option<Position> {
        position.x = position.x.checked_sub(1)?;
        Some(position)
    }

    fn east_of(&self, mut position: Position) -> Option<Position> {
        if position.x >= self.max_x {
            return None;
        }
        position.x += 1;
        Some(position)
    }

    fn get(&self, position: Position) -> Tile {
        self.inner[position.y][position.x]
    }

    pub fn enclosed_tiles_count(&self) -> Result<usize> {
        const INTERSECT_IGNORE_PIPES: [Pipe; 3] =
            [Pipe::NorthEast, Pipe::EastWest, Pipe::NorthWest];

        if !self.sanitized {
            bail!("Map must be sanitized before calling this function");
        }

        Ok(self.inner.iter().fold(0, |mut count, row| {
            let _ = row.iter().copied().fold(0, |mut intersect_count, tile| {
                match tile {
                    Tile::Pipe(pipe) if !INTERSECT_IGNORE_PIPES.contains(&pipe) => {
                        intersect_count += 1;
                    }
                    Tile::Ground if intersect_count % 2 != 0 => {
                        count += 1;
                    }
                    _ => (),
                }
                intersect_count
            });
            count
        }))
    }

    pub fn path(&self) -> Path {
        Path::new(self)
    }

    pub fn sanitize(&mut self) {
        if self.sanitized {
            return;
        }

        let mut empty_row = Vec::with_capacity(self.width);
        empty_row.extend(std::iter::repeat(Tile::Ground).take(self.width));

        let mut map = Vec::with_capacity(self.height);
        map.extend(std::iter::repeat(empty_row.clone()).take(self.height));

        for position in self.path() {
            map[position.y][position.x] = self.get(position);
        }

        *self = Map {
            inner: map,
            sanitized: true,
            ..*self
        }
    }
}

#[derive(Debug)]
pub struct Path<'a> {
    map: &'a Map,
    position: Position,
    direction: Direction,
    exhausted: bool,
}

impl<'a> Path<'a> {
    pub fn new(map: &'a Map) -> Self {
        Self {
            map,
            position: map.start,
            direction: map.get(map.start).as_pipe().directions()[0],
            exhausted: false,
        }
    }
}

impl Iterator for Path<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        let previous = self.position;
        self.position = self
            .map
            .to_direction_of(self.position, self.direction)
            .unwrap();
        if self.position == self.map.start {
            self.exhausted = true;
        }
        self.direction = self
            .map
            .get(self.position)
            .as_pipe()
            .directions()
            .iter()
            .copied()
            .find(|connected_direction| *connected_direction != self.direction.opposite())
            .unwrap();
        Some(previous)
    }
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
    pub fn all() -> [Direction; 4] {
        [North, East, South, West]
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    pub fn directions(self) -> [Direction; 2] {
        match self {
            NorthSouth => [North, South],
            EastWest => [East, West],
            NorthEast => [North, East],
            NorthWest => [North, West],
            SouthWest => [South, West],
            SouthEast => [East, South],
        }
    }
}

impl TryFrom<&[Direction]> for Pipe {
    type Error = Error;

    fn try_from(directions: &[Direction]) -> Result<Self> {
        if directions.len() != 2 {
            bail!(
                "Unexpected number of connected directions '{}'",
                directions.len()
            );
        }

        let mut directions: Vec<Direction> = directions.to_vec();

        directions.sort_unstable();

        Ok(match directions.as_slice() {
            [North, South] => NorthSouth,
            [East, West] => EastWest,
            [North, East] => NorthEast,
            [North, West] => NorthWest,
            [South, West] => SouthWest,
            [East, South] => SouthEast,
            _ => bail!(
                "Invalid combination of directions '[{:?}, {:?}]'",
                directions[0],
                directions[1]
            ),
        })
    }
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => bail!("Cannot convert character `{value}` to a tile"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Ground,
    Pipe(Pipe),
}

impl Tile {
    pub fn as_pipe(self) -> Pipe {
        match self {
            Tile::Pipe(pipe) => pipe,
            _ => panic!("as_pipe called on non-pipe tile!"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            'S' => Tile::Start,
            '.' => Tile::Ground,
            _ => Tile::Pipe(Pipe::try_from(value)?),
        })
    }
}
