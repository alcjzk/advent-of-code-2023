use anyhow::{anyhow, Result};
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
    fn energized_count(&self) -> usize;
}

impl LightMap for Map2D<Value> {
    fn is_set(&self, x: usize, y: usize, direction: Direction) -> bool {
        self[y][x][usize::from(direction)]
    }
    fn set(&mut self, x: usize, y: usize, direction: Direction) {
        self[y][x][usize::from(direction)] = true;
    }
    fn energized_count(&self) -> usize {
        self.rows().fold(0, |count, row| {
            row.iter()
                .filter(|values| values.iter().copied().any(std::convert::identity))
                .count()
                + count
        })
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

fn run(grid: &Grid, initial_beam: Beam) -> Result<usize> {
    let mut light_map: Map2D<Value> = Map2D::new(grid.height(), grid.width());
    let mut beams = vec![initial_beam];

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

    let count = light_map.energized_count();
    Ok(count)
}

fn part_one(grid: &Grid) -> Result<usize> {
    let initial_beam = Beam {
        x: 0,
        y: 0,
        direction: Right,
    };

    let count = run(grid, initial_beam)?;
    Ok(count)
}

fn part_two(grid: &Grid) -> Result<usize> {
    let mut initial_beams = vec![];

    let x_max = grid.width() - 1;
    let y_max = grid.height() - 1;

    initial_beams.extend((0..grid.width()).flat_map(|x| {
        [
            Beam {
                x,
                y: 0,
                direction: Down,
            },
            Beam {
                x,
                y: y_max,
                direction: Up,
            },
        ]
    }));

    initial_beams.extend((0..grid.height()).flat_map(|y| {
        [
            Beam {
                x: 0,
                y,
                direction: Right,
            },
            Beam {
                x: x_max,
                y,
                direction: Left,
            },
        ]
    }));

    let values = initial_beams.into_iter().map(|beam| run(grid, beam));
    let values: Vec<usize> = Result::from_iter(values)?;
    let max = values
        .into_iter()
        .max()
        .ok_or(anyhow!("no max value found"))?;

    Ok(max)
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;
    let grid = Grid::from_reader(file)?;

    println!("part one answer: {}", part_one(&grid)?);
    println!("part two answer: {}", part_two(&grid)?);

    Ok(())
}
