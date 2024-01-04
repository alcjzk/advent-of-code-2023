use anyhow::{Result, Error, anyhow, bail};
use std::io::{Read, BufRead, BufReader};
use std::fmt::{self, Write, Display};
use std::slice::Iter;
use std::ops::{Deref, Range};

#[derive(Debug)]
pub struct Columns<'a, T> {
    map: &'a Map2D<T>,
    x: Range<usize>,
}

impl<'a, T> Columns<'a, T> {
    fn new(map: &'a Map2D<T>) -> Self {
        let x = Range { start: 0, end: map.width };
        Self {
            map, x,
        }
    }
}

impl<'a, T> Iterator for Columns<'a, T> {
    type Item = Column<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.x.next() {
            return Some(Column::new(self.map, x));
        }
        None
    }
}

impl<T> DoubleEndedIterator for Columns<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.x.next_back() {
            return Some(Column::new(self.map, x));
        }
        None
    }
}

#[derive(Debug)]
pub struct Column<'a, T> {
    map: &'a Map2D<T>,
    x: usize,
    y: Range<usize>,
}

impl<'a, T> Column<'a, T> {
    fn new(map: &'a Map2D<T>, x: usize) -> Self {
        let y = Range { start: 0, end: map.height };

        Self {
            map, x, y,
        }
    }
}

impl<'a, T> Iterator for Column<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.y.next() {
            return Some(&self.map[y][self.x]);
        }
        None
    }
}

impl<T> DoubleEndedIterator for Column<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.y.next_back() {
            return Some(&self.map[y][self.x]);
        }
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Row<T> {
    inner: Box<[T]>,
}

impl<T> Row<T> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn iter(&self) -> Iter<'_, T> {
        self.inner.iter()
    }
}

impl<T> Deref for Row<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Display> Display for Row<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self {
            cell.fmt(f)?;
        }
        Ok(())
    }
}

impl<'a, T> IntoIterator for &'a Row<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: TryFrom<char>> TryFrom<&str> for Row<T>
where
    <T as TryFrom<char>>::Error: Into<Error>,
{
    type Error = Error;

    fn try_from(line: &str) -> Result<Self> {
        let tiles = line
            .chars()
            .map(|character| -> Result<T> {
                T::try_from(character)
                    .map_err(|error| error.into())
            });

        let inner = Result::from_iter(tiles)?;

        Ok(Self { inner })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map2D<T> {
    inner: Box<[Row<T>]>,
    height: usize,
    width: usize,
}

impl<T> Map2D<T> {
    pub fn rows(&self) -> impl Iterator<Item = &Row<T>> {
        self.inner.iter()
    }
    pub fn column(&self, index: usize) -> Option<Column<'_, T>> {
        if index >= self.width {
            return None;
        }
        Some(Column::new(self, index))
    }
    pub fn columns(&self) -> Columns<'_, T> {
        Columns::new(self)
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self[y][x])
    }
}

impl<T> Deref for Map2D<T> {
    type Target = [Row<T>];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: TryFrom<char>> Map2D<T>
where
    <T as TryFrom<char>>::Error: Into<Error>,
{
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let mut lines = BufReader::new(reader).lines().peekable();

        let width = lines
            .peek_mut()
            .ok_or(anyhow!("no line in reader"))?
            .as_ref()
            .map_err(|error| anyhow!("error reading line: {error}"))
            .map(|s| s.len())?;

        let inner: Box<[Row<T>]> = lines
            .enumerate()
            .map(|(n, maybe_line)| {
                let row = Row::try_from(maybe_line?.as_str())?;

                if row.len() != width {
                    bail!("incorrect row width '{}' on line {n}, expected '{width}'", row.len());
                }

                Ok(row)
            })
            .collect::<Result<_>>()?;

        let height = inner.len();
        
        Ok(Self {
            inner,
            height,
            width,
        })
    }
}

impl<T: Display> Display for Map2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            row.fmt(f)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}
