use anyhow::{bail, Error, Result};
use std::fmt::{self, Write};
use std::io::{BufRead, BufReader, Read};

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

        Ok(Self { inner })
    }

    fn is_all_spaces(&self, column_index: usize) -> bool {
        let height = self.inner.len();

        for row_index in 0..height {
            if self.inner[row_index][column_index] != Cell::Space {
                return false;
            }
        }
        true
    }

    pub fn expand(&mut self) {
        self.inner =
            self.inner
                .iter()
                .fold(Vec::with_capacity(self.inner.len()), |mut rows, row| {
                    if row.iter().copied().all(|cell| cell == Cell::Space) {
                        rows.extend(std::iter::repeat(row.clone()).take(2));
                    } else {
                        rows.push(row.clone());
                    }
                    rows
                });

        // FIXME: Better way to do this?
        let mut column_index = 0;
        loop {
            if column_index >= self.inner[0].len() {
                break;
            }
            if self.is_all_spaces(column_index) {
                for row in self.inner.iter_mut() {
                    row.insert(column_index, Cell::Space);
                }
                column_index += 1;
            }
            column_index += 1;
        }
    }

    pub fn galaxies(&self) -> impl Iterator<Item = Position> + '_ {
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
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn distance_to(self, other: Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Space,
    Galaxy,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        use Cell::*;

        Ok(match value {
            '.' => Space,
            '#' => Galaxy,
            _ => bail!("Cannot convert '{value}' to a Cell"),
        })
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        use Cell::*;

        match cell {
            Space => '.',
            Galaxy => '#',
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
    fn universe_expand() -> Result<()> {
        let mut universe = Universe::from_reader(OpenOptions::new().read(true).open("test")?)?;
        let universe_expanded =
            Universe::from_reader(OpenOptions::new().read(true).open("test_expanded")?)?;

        universe.expand();

        assert_eq!(universe, universe_expanded);

        Ok(())
    }
}
