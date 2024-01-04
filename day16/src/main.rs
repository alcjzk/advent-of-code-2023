use anyhow::Result;
use macros::char_enum;
use map2d::Map2D;
use std::{fmt::Write, fs::OpenOptions};
use Direction::*;

const ITERATIONS_MAX: usize = 100_000;

type Grid = Map2D<Tile>;

char_enum! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    Tile {
        Space => '.',
        MirrorFw => '/',
        MirrorBw => '\\',
        SplitterVertical => '|',
        SplitterHorizontal => '-',
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Up => Some((x, y.checked_sub(1)?)),
            Down => Some((x, y.checked_add(1)?)),
            Left => Some((x.checked_sub(1)?, y)),
            Right => Some((x.checked_add(1)?, y)),
        }
    }
    fn turn(self, tile: Tile) -> (Direction, Option<Direction>) {
        match (self, tile) {
            (Up, Tile::MirrorFw) => (Right, None),
            (Down, Tile::MirrorFw) => (Left, None),
            (Left, Tile::MirrorFw) => (Down, None),
            (Right, Tile::MirrorFw) => (Up, None),

            (Up, Tile::MirrorBw) => (Left, None),
            (Down, Tile::MirrorBw) => (Right, None),
            (Left, Tile::MirrorBw) => (Up, None),
            (Right, Tile::MirrorBw) => (Down, None),

            (Right, Tile::SplitterVertical) | (Left, Tile::SplitterVertical) => (Up, Some(Down)),

            (Up, Tile::SplitterHorizontal) | (Down, Tile::SplitterHorizontal) => {
                (Left, Some(Right))
            }
            _ => (self, None),
        }
    }
}

impl From<Direction> for usize {
    fn from(direction: Direction) -> usize {
        match direction {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3,
        }
    }
}

trait LightMap {
    fn is_set(&self, x: usize, y: usize, direction: Direction) -> bool;
    fn set(&mut self, x: usize, y: usize, direction: Direction);
}

impl LightMap for Map2D<Value> {
    fn is_set(&self, x: usize, y: usize, direction: Direction) -> bool {
        self[y][x][usize::from(direction)]
    }
    fn set(&mut self, x: usize, y: usize, direction: Direction) {
        self[y][x][usize::from(direction)] = true;
    }
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Default, Clone, Copy)]
struct Value([bool; 4]);

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.iter().copied().any(std::convert::identity) {
            f.write_char('#')?;
        } else {
            f.write_char('.')?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Value {
    type Target = [bool; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;

    let grid = Grid::from_reader(file)?;

    let mut light_map: Map2D<Value> = Map2D::new(grid.height(), grid.width());

    let mut beams = vec![Beam {
        x: 0,
        y: 0,
        direction: Right,
    }];

    for _ in 0..ITERATIONS_MAX {
        let mut new_beams = vec![];
        beams.retain_mut(|beam| {
            if let Some(tile) = grid.get(beam.x, beam.y) {
                if light_map.is_set(beam.x, beam.y, beam.direction) {
                    return false;
                }
                light_map.set(beam.x, beam.y, beam.direction);
                let (direction, other) = beam.direction.turn(*tile);
                if let Some(other) = other {
                    if let Some((x, y)) = other.next(beam.x, beam.y) {
                        new_beams.push(Beam {
                            x,
                            y,
                            direction: other,
                        });
                    }
                }
                beam.direction = direction;
                if let Some((x, y)) = direction.next(beam.x, beam.y) {
                    beam.x = x;
                    beam.y = y;
                } else {
                    return false;
                }
            }
            true
        });
        beams.extend(new_beams.into_iter());
        if beams.is_empty() {
            break;
        }
    }

    let mut count = 0;

    for row in light_map.rows() {
        for light_values in row {
            if light_values.iter().copied().any(std::convert::identity) {
                count += 1;
            }
        }
    }

    println!("{count}");

    Ok(())
}
