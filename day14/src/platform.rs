use crate::node::Node;
use anyhow::Result;
use std::fmt::{self, Write};
use std::io::{BufRead, BufReader, Read};
use std::ops::{self, Range};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Platform {
    inner: Box<[PlatformRow]>,
    width: usize,
}

impl Platform {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let inner: Box<[PlatformRow]> = Result::from_iter(
            BufReader::new(reader)
                .lines()
                .map(|maybe_line| PlatformRow::from_line(&maybe_line?)),
        )?;
        let width = inner.first().map_or(0, |row| row.len());

        Ok(Self { inner, width })
    }

    pub fn load(&self) -> usize {
        self.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .copied()
                    .filter(|node| *node == Node::RoundedRock)
                    .count()
                    * (self.len() - y)
            })
            .sum()
    }

    pub fn tilt_north(self) -> Self {
        fn _place_rocks(platform: &mut Platform, x: usize, offset: usize, count: usize) {
            platform
                .iter_mut()
                .skip(offset)
                .take(count)
                .for_each(|row| row[x] = Node::RoundedRock);
        }

        let mut out = self.clone();

        for (x, column) in self.columns().enumerate() {
            let mut offset = 0;
            let mut round_rock_count = 0;

            for (y, node) in column {
                out[y][x] = Node::Space;

                match node {
                    Node::RoundedRock => {
                        round_rock_count += 1;
                    }
                    Node::CubeShapedRock => {
                        _place_rocks(&mut out, x, offset, round_rock_count);
                        out[y][x] = Node::CubeShapedRock;
                        offset = y + 1;
                        round_rock_count = 0;
                    }
                    _ => (),
                }
            }
            _place_rocks(&mut out, x, offset, round_rock_count);
        }
        out
    }

    pub fn tilt_south(self) -> Self {
        fn _place_rocks(platform: &mut Platform, x: usize, offset: usize, count: usize) {
            platform
                .iter_mut()
                .rev()
                .skip(offset)
                .take(count)
                .for_each(|row| row[x] = Node::RoundedRock);
        }

        let mut out = self.clone();

        for (x, column) in self.columns().enumerate() {
            let mut offset = 0;
            let mut round_rock_count = 0;

            for (y, node) in column.rev() {
                out[y][x] = Node::Space;
                match node {
                    Node::RoundedRock => {
                        round_rock_count += 1;
                    }
                    Node::CubeShapedRock => {
                        _place_rocks(&mut out, x, offset, round_rock_count);
                        out[y][x] = Node::CubeShapedRock;
                        offset = self.len() - y;
                        round_rock_count = 0;
                    }
                    _ => (),
                }
            }
            _place_rocks(&mut out, x, offset, round_rock_count);
        }
        out
    }

    pub fn tilt_west(self) -> Self {
        fn _place_rocks(platform: &mut Platform, y: usize, offset: usize, count: usize) {
            platform[y]
                .iter_mut()
                .skip(offset)
                .take(count)
                .for_each(|node| *node = Node::RoundedRock);
        }

        let mut out = self.clone();

        for (y, row) in self.iter().enumerate() {
            let mut offset = 0;
            let mut round_rock_count = 0;

            for (x, node) in row.node_indices() {
                out[y][x] = Node::Space;

                match node {
                    Node::RoundedRock => {
                        round_rock_count += 1;
                    }
                    Node::CubeShapedRock => {
                        _place_rocks(&mut out, y, offset, round_rock_count);
                        out[y][x] = Node::CubeShapedRock;
                        offset = x + 1;
                        round_rock_count = 0;
                    }
                    _ => (),
                }
            }
            _place_rocks(&mut out, y, offset, round_rock_count);
        }
        out
    }

    pub fn tilt_east(self) -> Self {
        fn _place_rocks(platform: &mut Platform, y: usize, offset: usize, count: usize) {
            platform[y]
                .iter_mut()
                .rev()
                .skip(offset)
                .take(count)
                .for_each(|node| *node = Node::RoundedRock);
        }

        let mut out = self.clone();

        for (y, row) in self.iter().enumerate() {
            let mut offset = 0;
            let mut round_rock_count = 0;

            for (x, node) in row.node_indices().rev() {
                out[y][x] = Node::Space;

                match node {
                    Node::RoundedRock => {
                        round_rock_count += 1;
                    }
                    Node::CubeShapedRock => {
                        _place_rocks(&mut out, y, offset, round_rock_count);
                        out[y][x] = Node::CubeShapedRock;
                        offset = row.len() - x;
                        round_rock_count = 0;
                    }
                    _ => (),
                }
            }
            _place_rocks(&mut out, y, offset, round_rock_count);
        }
        out
    }

    pub fn spin(self) -> Self {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }

    pub fn spin_n(self, n: usize) -> Self {
        let mut pattern = self;
        for _ in 0..n {
            pattern = pattern.spin();
        }
        pattern
    }

    fn columns(&self) -> Columns<'_> {
        Columns::new(self)
    }

    #[cfg(test)]
    fn from_literal(literal: &str) -> Self {
        let inner: Box<[PlatformRow]> =
            Result::from_iter(literal.split_terminator('\n').map(PlatformRow::from_line)).unwrap();
        let width = inner.first().map_or(0, |row| row.len());

        Self { inner, width }
    }
}

impl ops::Deref for Platform {
    type Target = [PlatformRow];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug)]
struct ColumnIndices<'a> {
    platform: &'a Platform,
    x: usize,
    y: Range<usize>,
}

impl<'a> ColumnIndices<'a> {
    fn new(platform: &'a Platform, x: usize) -> Self {
        let y = Range {
            start: 0,
            end: platform.len(),
        };

        Self { platform, x, y }
    }
}

impl Iterator for ColumnIndices<'_> {
    type Item = (usize, Node);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.y.next() {
            return Some((y, self.platform[y][self.x]));
        }
        None
    }
}

impl ExactSizeIterator for ColumnIndices<'_> {
    fn len(&self) -> usize {
        self.platform.len()
    }
}

impl DoubleEndedIterator for ColumnIndices<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(y) = self.y.next_back() {
            return Some((y, self.platform[y][self.x]));
        }
        None
    }
}

#[derive(Debug)]
struct Columns<'a> {
    platform: &'a Platform,
    x: Range<usize>,
}

impl<'a> Columns<'a> {
    fn new(platform: &'a Platform) -> Self {
        Self {
            platform,
            x: Range {
                start: 0,
                end: platform.width,
            },
        }
    }
}

impl<'a> Iterator for Columns<'a> {
    type Item = ColumnIndices<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.x.next() {
            return Some(ColumnIndices::new(self.platform, x));
        }
        None
    }
}

impl ExactSizeIterator for Columns<'_> {
    fn len(&self) -> usize {
        self.x.len()
    }
}

impl FromIterator<PlatformRow> for Platform {
    fn from_iter<T: IntoIterator<Item = PlatformRow>>(iter: T) -> Self {
        let inner: Box<[PlatformRow]> = iter.into_iter().collect();
        let width = inner.first().map_or(0, |row| row.len());

        Self { inner, width }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.iter() {
            row.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRow {
    inner: Box<[Node]>,
}

impl PlatformRow {
    fn from_line(line: &str) -> Result<Self> {
        line.chars().map(Node::try_from).collect()
    }
    fn nodes(&self) -> impl Iterator<Item = Node> + '_ {
        self.iter().copied()
    }
    fn node_indices(&self) -> NodeIndices<'_> {
        NodeIndices::new(self)
    }
}

impl FromIterator<Node> for PlatformRow {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl ops::Deref for PlatformRow {
    type Target = [Node];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for PlatformRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl fmt::Display for PlatformRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.nodes() {
            node.fmt(f)?;
        }
        f.write_char('\n')
    }
}

#[derive(Debug)]
struct NodeIndices<'a> {
    row: &'a PlatformRow,
    range: Range<usize>,
}

impl<'a> NodeIndices<'a> {
    fn new(row: &'a PlatformRow) -> Self {
        Self {
            row,
            range: Range {
                start: 0,
                end: row.len(),
            },
        }
    }
}

impl Iterator for NodeIndices<'_> {
    type Item = (usize, Node);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.range.next() {
            return Some((x, self.row[x]));
        }
        None
    }
}

impl ExactSizeIterator for NodeIndices<'_> {
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl DoubleEndedIterator for NodeIndices<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.range.next_back() {
            return Some((x, self.row[x]));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn example_literal() -> &'static str {
        indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "}
    }

    fn example_literal_tilted() -> &'static str {
        indoc! {"
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....
        "}
    }

    fn example_literal_spin1() -> &'static str {
        indoc! {"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
        "}
    }

    fn example_literal_spin2() -> &'static str {
        indoc! {"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
        "}
    }

    fn example_literal_spin3() -> &'static str {
        indoc! {"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
        "}
    }

    fn example_platform() -> Platform {
        Platform::from_literal(example_literal())
    }

    fn example_platform_tilted() -> Platform {
        Platform::from_literal(example_literal_tilted())
    }

    #[test]
    fn load() {
        assert_eq!(example_platform().tilt_north().load(), 136);
    }

    #[test]
    fn literal_display() {
        assert_eq!(example_literal(), example_platform().to_string());
    }

    #[test]
    fn tilt() {
        assert_eq!(
            example_platform().tilt_north().to_string(),
            example_platform_tilted().to_string()
        )
    }

    #[test]
    fn spin() {
        let pattern = example_platform();
        assert_eq!(pattern.to_string(), example_literal());
        let spin1 = pattern.spin();
        assert_eq!(spin1.to_string(), example_literal_spin1());
        let spin2 = spin1.spin();
        assert_eq!(spin2.to_string(), example_literal_spin2());
        let spin3 = spin2.spin();
        assert_eq!(spin3.to_string(), example_literal_spin3());
    }
}
