use anyhow::{anyhow, bail, Error, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::num::NonZeroUsize;
use std::rc::Rc;
use std::write;

use Spring::*;

type GroupSize = NonZeroUsize;
type Map = HashMap<Input, usize>;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = Error;

    fn try_from(character: char) -> Result<Self> {
        Ok(match character {
            '#' => Damaged,
            '.' => Operational,
            '?' => Unknown,
            _ => bail!("Cannot convert character '{character}' to a spring"),
        })
    }
}

impl From<Spring> for char {
    fn from(spring: Spring) -> Self {
        match spring {
            Damaged => '#',
            Operational => '.',
            Unknown => '?',
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Input {
    pattern: Rc<[Spring]>,
    group_sizes: Rc<[GroupSize]>,
}

impl Input {
    pub fn new(line: &str) -> Result<Self> {
        Self::try_from(line)
    }

    fn from_parts<P, G>(pattern: P, group_sizes: G) -> Self
    where
        P: Into<Rc<[Spring]>>,
        G: Into<Rc<[GroupSize]>>,
    {
        Self {
            pattern: pattern.into(),
            group_sizes: group_sizes.into(),
        }
    }

    pub fn arrangement_count(&self) -> usize {
        let mut map = Map::new();

        self.arrangement_count_cached(&mut map)
    }

    fn arrangement_count_cached(&self, map: &mut Map) -> usize {
        if let Some(value) = map.get(self).copied() {
            return value;
        }
        let value = self.arrangement_count_cached_impl(map);

        map.insert(self.clone(), value);
        value
    }

    fn arrangement_count_cached_impl(&self, map: &mut Map) -> usize {
        let group_size = match self.group_sizes.first().copied() {
            Some(group_size) => usize::from(group_size),
            None if !self.pattern.contains(&Damaged) => return 1,
            None => return 0,
        };

        let spring = match self.pattern.first().copied() {
            Some(spring) => spring,
            None => return 0,
        };

        let count = match spring {
            Operational => {
                let input = Input::from_parts(&self.pattern[1..], self.group_sizes.clone());
                return input.arrangement_count_cached(map);
            }
            Unknown => {
                let input = Input::from_parts(&self.pattern[1..], self.group_sizes.clone());
                input.arrangement_count_cached(map)
            }
            Damaged => 0,
        };

        if self
            .pattern
            .get(1..group_size)
            .map_or(true, |pattern| pattern.contains(&Operational))
        {
            return count;
        }

        match self.pattern.get(group_size) {
            Some(Damaged) => count,
            Some(_) => {
                let input =
                    Input::from_parts(&self.pattern[group_size + 1..], &self.group_sizes[1..]);
                count + input.arrangement_count_cached(map)
            }
            None if self.group_sizes.len() == 1 => count + 1,
            None => count,
        }
    }

    pub fn unfold(&self) -> Self {
        let pattern = Itertools::intersperse(
            std::iter::repeat(self.pattern.iter().copied()).take(5),
            [Unknown].iter().copied(),
        )
        .flatten()
        .collect();

        let group_sizes = std::iter::repeat(self.group_sizes.iter().copied())
            .take(5)
            .flatten()
            .collect();

        Self {
            pattern,
            group_sizes,
        }
    }
}

impl TryFrom<&str> for Input {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self> {
        let mut split = line.split_ascii_whitespace();

        let pattern = Result::from_iter(
            split
                .next()
                .ok_or(anyhow!("Invalid input '{line}'"))?
                .chars()
                .map(Spring::try_from),
        )?;

        let group_sizes = Result::from_iter(
            split
                .next()
                .ok_or(anyhow!("Invalid input '{line}'"))?
                .split(',')
                .map(str::parse::<NonZeroUsize>),
        )?;

        Ok(Self {
            pattern,
            group_sizes,
        })
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for spring in self.pattern.iter().copied() {
            f.write_char(spring.into())?;
        }
        f.write_char(' ')?;
        if let Some(group_size) = self.group_sizes.first() {
            write!(f, "{group_size}")?;
        }
        if let Some(group_sizes) = self.group_sizes.get(1..) {
            for group_size in group_sizes {
                write!(f, ",{group_size}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() -> Result<()> {
        assert_eq!(1, Input::new("???.### 1,1,3")?.arrangement_count());
        assert_eq!(4, Input::new(".??..??...?##. 1,1,3")?.arrangement_count());
        assert_eq!(
            1,
            Input::new("?#?#?#?#?#?#?#? 1,3,1,6")?.arrangement_count()
        );
        assert_eq!(1, Input::new("????.#...#... 4,1,1")?.arrangement_count());
        assert_eq!(
            4,
            Input::new("????.######..#####. 1,6,5")?.arrangement_count()
        );
        assert_eq!(10, Input::new("?###???????? 3,2,1")?.arrangement_count());

        Ok(())
    }

    #[test]
    fn part_two() -> Result<()> {
        assert_eq!(1, Input::new("???.### 1,1,3")?.unfold().arrangement_count());
        assert_eq!(
            16384,
            Input::new(".??..??...?##. 1,1,3")?
                .unfold()
                .arrangement_count()
        );
        assert_eq!(
            1,
            Input::new("?#?#?#?#?#?#?#? 1,3,1,6")?
                .unfold()
                .arrangement_count()
        );
        assert_eq!(
            16,
            Input::new("????.#...#... 4,1,1")?
                .unfold()
                .arrangement_count()
        );
        assert_eq!(
            2500,
            Input::new("????.######..#####. 1,6,5")?
                .unfold()
                .arrangement_count()
        );
        assert_eq!(
            506250,
            Input::new("?###???????? 3,2,1")?
                .unfold()
                .arrangement_count()
        );

        Ok(())
    }
}
