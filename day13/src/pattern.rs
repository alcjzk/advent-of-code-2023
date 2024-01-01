use anyhow::{bail, Error, Result};
use itertools::Itertools;
use std::fmt::{self, Write};
use std::io::{BufRead, BufReader, Read};
use std::ops;

use Terrain::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Terrain {
    Ash,
    Rock,
}

impl TryFrom<char> for Terrain {
    type Error = Error;

    fn try_from(character: char) -> Result<Self> {
        Ok(match character {
            '#' => Rock,
            '.' => Ash,
            _ => bail!("Cannot convert character '{character}' to Terrain"),
        })
    }
}

impl From<Terrain> for char {
    fn from(terrain: Terrain) -> Self {
        match terrain {
            Rock => '#',
            Ash => '.',
        }
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char((*self).into())
    }
}

#[derive(Debug)]
pub struct PatternRow {
    inner: Box<[Terrain]>,
}

impl PatternRow {
    fn reflection_pairs(&self, position: usize) -> impl Iterator<Item = (Terrain, Terrain)> + '_ {
        let left = self.inner[..position].iter().copied().rev();
        let right = self.inner[position..].iter().copied();
        left.zip(right)
    }
    fn is_reflection_at(&self, position: usize) -> bool {
        self.reflection_pairs(position).all(|(a, b)| a == b)
    }
    fn reflection_errors_count(&self, position: usize) -> usize {
        self.reflection_pairs(position)
            .filter(|(a, b)| a != b)
            .count()
    }
}

impl FromIterator<Terrain> for PatternRow {
    fn from_iter<T: IntoIterator<Item = Terrain>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl ops::Deref for PatternRow {
    type Target = [Terrain];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub struct Pattern {
    inner: Box<[PatternRow]>,
    height: usize,
    width: usize,
}

impl Pattern {
    #[cfg(test)]
    fn from_literal(literal: &str) -> Result<Self> {
        let inner: Box<[PatternRow]> = Result::from_iter(
            literal
                .split_terminator('\n')
                .map(|row| Result::from_iter(row.chars().map(Terrain::try_from))),
        )?;
        let height = inner.len();
        let width = inner[0].len();
        Ok(Self {
            inner,
            height,
            width,
        })
    }
    fn reflection(&self) -> Option<usize> {
        self.reflection_positions()
            .find(|position| self.is_reflection_at(*position))
    }
    fn reflection_corrected(&self) -> Option<usize> {
        self.reflection_positions()
            .find(|position| self.is_reflection_at_with_error(*position))
    }
    fn is_reflection_at_with_error(&self, position: usize) -> bool {
        self.inner
            .iter()
            .filter(|row| row.reflection_errors_count(position) == 1)
            .count()
            == 1
            && self
                .inner
                .iter()
                .all(|row| row.reflection_errors_count(position) <= 1)
    }
    fn is_reflection_at(&self, position: usize) -> bool {
        self.inner.iter().all(|row| row.is_reflection_at(position))
    }
    fn reflection_positions(&self) -> impl Iterator<Item = usize> + '_ {
        (1..self.width / 2)
            .rev()
            .interleave(self.width / 2..self.width)
    }
    fn transpose(&self) -> Self {
        let width = self.height;
        let height = self.width;

        let inner = (0..height)
            .map(|y| (0..width).map(|x| self.inner[x][y]).collect())
            .collect();
        Pattern {
            height: self.width,
            width: self.height,
            inner,
        }
    }
}

impl From<Box<[PatternRow]>> for Pattern {
    fn from(inner: Box<[PatternRow]>) -> Self {
        let height = inner.len();
        let width = inner[0].len();
        Self {
            inner,
            height,
            width,
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.inner.iter() {
            for terrain in row.iter() {
                terrain.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Patterns {
    inner: Box<[Pattern]>,
}

impl Patterns {
    pub fn read<R: Read>(reader: R) -> Result<Self> {
        let mut lines = BufReader::new(reader).lines();
        let mut patterns = vec![];
        loop {
            let pattern: Box<[PatternRow]> = lines
                .by_ref()
                .map_while(|maybe_line| {
                    let line = maybe_line.ok().filter(|line| !line.is_empty())?;
                    Some(Result::from_iter(line.chars().map(Terrain::try_from)).ok()?)
                })
                .collect();

            if pattern.is_empty() {
                return Ok(Self {
                    inner: patterns.into(),
                });
            }
            patterns.push(pattern.into());
        }
    }
    pub fn summarize(&self) -> usize {
        let (vertical, horizontal) =
            self.inner
                .iter()
                .fold((0, 0), |(mut vertical, mut horizontal), pattern| {
                    match pattern.reflection() {
                        Some(position) => vertical += position,
                        None => horizontal += pattern.transpose().reflection().unwrap(),
                    }
                    (vertical, horizontal)
                });
        vertical + horizontal * 100
    }
    pub fn summarize2(&self) -> usize {
        let (vertical, horizontal) =
            self.inner
                .iter()
                .fold((0, 0), |(mut vertical, mut horizontal), pattern| {
                    match pattern.reflection_corrected() {
                        Some(position) => vertical += position,
                        None => horizontal += pattern.transpose().reflection_corrected().unwrap(),
                    }
                    (vertical, horizontal)
                });
        vertical + horizontal * 100
    }
}

impl FromIterator<Pattern> for Patterns {
    fn from_iter<T: IntoIterator<Item = Pattern>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const fn pattern1_literal() -> &'static str {
        indoc! {r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        "#}
    }

    const fn pattern2_literal() -> &'static str {
        indoc! {r#"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "#}
    }

    fn pattern1() -> Result<Pattern> {
        Pattern::from_literal(pattern1_literal())
    }

    fn pattern2() -> Result<Pattern> {
        Pattern::from_literal(pattern2_literal())
    }

    #[test]
    fn pattern_literal_display() -> Result<()> {
        assert_eq!(pattern1()?.to_string(), pattern1_literal());
        assert_eq!(pattern2()?.to_string(), pattern2_literal());
        Ok(())
    }

    #[test]
    fn patterns_summarize() -> Result<()> {
        let patterns = Patterns::from_iter([pattern1()?, pattern2()?]);
        assert_eq!(patterns.summarize(), 405);
        assert_eq!(patterns.summarize2(), 400);
        Ok(())
    }
}
