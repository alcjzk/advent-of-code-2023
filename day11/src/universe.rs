use anyhow::{bail, Error, Result};
use itertools::Itertools;
use std::fmt::{self, Write};
use std::io::{BufRead, BufReader, Read};

use Cell::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Universe {
    inner: Vec<Vec<Cell>>,
}

impl Universe {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let inner = BufReader::new(reader)
            .lines()
            .map(|maybe_line| {
                maybe_line?
                    .chars()
                    .map(Cell::try_from)
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?;

        let mut universe = Self { inner };

        universe.expand();

        Ok(universe)
    }

    fn distance(&self, a: Position, b: Position, void_size: usize) -> usize {
        let mut distance = 0;
        for x in a.x.min(b.x)..a.x.max(b.x) {
            match self.get(Position { x, y: a.y }).is_horizontal_void() {
                true => distance += void_size,
                false => distance += 1,
            }
        }
        for y in a.y.min(b.y)..a.y.max(b.y) {
            match self.get(Position { x: a.x, y }).is_vertical_void() {
                true => distance += void_size,
                false => distance += 1,
            }
        }
        distance
    }

    fn get(&self, position: Position) -> Cell {
        self.inner[position.y][position.x]
    }

    pub fn distances(&self, void_size: usize) -> impl Iterator<Item = usize> + '_ {
        self.galaxies()
            .combinations(2)
            .map(move |pair| self.distance(pair[0], pair[1], void_size))
    }

    fn expand(&mut self) {
        self.inner
            .iter_mut()
            .filter(|row| row.iter().all(|cell| *cell == Cell::Space))
            .for_each(|row| row.iter_mut().for_each(|cell| *cell = Cell::VerticalVoid));

        for column_idx in 0..self.inner[0].len() {
            if self.column_cells(column_idx).all(Cell::is_space) {
                self.column_cells_mut(column_idx).for_each(|cell| {
                    *cell = match cell {
                        Cell::VerticalVoid => Cell::Void,
                        _ => Cell::HorizontalVoid,
                    }
                });
            }
        }
    }

    fn column_cells_mut(&mut self, index: usize) -> impl Iterator<Item = &mut Cell> + '_ {
        self.inner
            .iter_mut()
            .map(move |row| row.get_mut(index).unwrap())
    }

    fn column_cells(&self, index: usize) -> impl Iterator<Item = Cell> + '_ {
        self.inner
            .iter()
            .map(move |row| row.get(index).copied().unwrap())
    }

    fn galaxies(&self) -> impl Iterator<Item = Position> + '_ {
        self.inner.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter_map(move |(x, cell)| match cell {
                    Cell::Galaxy => Some(Position { x, y }),
                    _ => None,
                })
        })
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.inner.iter() {
            for cell in row {
                write!(f, "{cell}")?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Space,
    Galaxy,
    HorizontalVoid,
    VerticalVoid,
    Void,
}

impl Cell {
    fn is_vertical_void(self) -> bool {
        self == VerticalVoid || self == Void
    }
    fn is_horizontal_void(self) -> bool {
        self == HorizontalVoid || self == Void
    }
    fn is_space(self) -> bool {
        !matches!(self, Cell::Galaxy)
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '.' => Space,
            '#' => Galaxy,
            _ => bail!("Cannot convert '{value}' to a Cell"),
        })
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Space => '.',
            Galaxy => '#',
            _ => '+',
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(char::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::OpenOptions;

    #[test]
    fn distances() -> Result<()> {
        let mut universe = Universe::from_reader(OpenOptions::new().read(true).open("test")?)?;
        universe.expand();

        assert_eq!(
            universe.distance(Position { x: 3, y: 0 }, Position { x: 7, y: 8 }, 2),
            15
        );
        assert_eq!(
            universe.distance(Position { x: 0, y: 2 }, Position { x: 9, y: 6 }, 2),
            17
        );

        assert_eq!(universe.distances(10).sum::<usize>(), 1030);
        assert_eq!(universe.distances(100).sum::<usize>(), 8410);

        Ok(())
    }
}
