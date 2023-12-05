use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Schematic(Vec<Vec<char>>);

impl Schematic {
    fn value(&self, point: Point) -> Option<char> {
        self.0.get(point.y as usize)?.get(point.x as usize).copied()
    }
    fn value_mut(&mut self, point: Point) -> Option<&mut char> {
        self.0.get_mut(point.y as usize)?.get_mut(point.x as usize)
    }
    fn point_values(&self) -> PointValues<'_> {
        PointValues::new(self)
    }
    fn symbols(&self) -> Symbols<'_> {
        Symbols::new(self)
    }
}

impl FromIterator<String> for Schematic {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Schematic(
            iter.into_iter()
                .map(|string| string.chars().collect())
                .collect(),
        )
    }
}

#[derive(Debug)]
struct PointValues<'a> {
    schematic: &'a Schematic,
    point: Point,
    is_exhausted: bool,
}

impl<'a> PointValues<'a> {
    fn new(schematic: &'a Schematic) -> Self {
        Self {
            schematic,
            point: Point::default(),
            is_exhausted: false,
        }
    }
}

impl Iterator for PointValues<'_> {
    type Item = (Point, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_exhausted {
            return None;
        }

        let value = match self.schematic.value(self.point) {
            Some(value) => value,
            None => {
                self.is_exhausted = true;
                return None;
            }
        };
        let point = self.point;
        if self.point.x < (self.schematic.0[self.point.y as usize].len() - 1) as u32 {
            self.point.x += 1;
        } else if self.point.y < (self.schematic.0.len() - 1) as u32 {
            self.point.x = 0;
            self.point.y += 1;
        } else {
            self.is_exhausted = true;
        }
        Some((point, value))
    }
}

#[derive(Debug)]
struct Symbols<'a> {
    point_values: PointValues<'a>,
}

impl<'a> Symbols<'a> {
    fn new(schematic: &'a Schematic) -> Self {
        Self {
            point_values: schematic.point_values(),
        }
    }
}

impl Iterator for Symbols<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        for (point, value) in self.point_values.by_ref() {
            if value != '.' && !value.is_ascii_digit() {
                return Some(point);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Adjacent {
    index: usize,
    point: Point,
}

impl Adjacent {
    fn new(point: Point) -> Self {
        Self { index: 0, point }
    }
}

impl Iterator for Adjacent {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        fn _point(index: usize, point: Point) -> Option<Point> {
            Some(match index {
                0 => point.up()?.right()?,
                1 => point.up()?,
                2 => point.up()?.left()?,
                3 => point.right()?,
                4 => point.left()?,
                5 => point.down()?.right()?,
                6 => point.down()?,
                7 => point.down()?.left()?,
                _ => unreachable!(),
            })
        }
        while self.index <= 7 {
            let point = _point(self.index, self.point);
            self.index += 1;
            if point.is_some() {
                return point;
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn left(self) -> Option<Point> {
        Some(Point {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }
    fn right(self) -> Option<Point> {
        Some(Point {
            x: self.x.checked_add(1)?,
            y: self.y,
        })
    }
    fn up(self) -> Option<Point> {
        Some(Point {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }
    fn down(self) -> Option<Point> {
        Some(Point {
            x: self.x,
            y: self.y.checked_add(1)?,
        })
    }
    fn adjacent(self) -> Adjacent {
        Adjacent::new(self)
    }
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("./input")?;
    let reader = BufReader::new(file);
    let mut schematic: Schematic = reader.lines().collect::<Result<_, _>>()?;
    let adjacent_points: Vec<_> = schematic
        .symbols()
        .flat_map(|symbol| symbol.adjacent())
        .collect();
    let sum: u32 = adjacent_points
        .into_iter()
        .filter_map(|point| {
            let value = schematic.value_mut(point).unwrap();
            let mut num = match value.to_digit(10) {
                Some(digit) => {
                    *value = '.';
                    digit
                }
                _ => return None,
            };
            let mut mul = 10;
            let mut start = point;
            let mut end = point;
            while let Some(point) = start.left() {
                let value = match schematic.value_mut(point) {
                    Some(value) => value,
                    None => break,
                };
                let digit = match value.to_digit(10) {
                    Some(digit) => {
                        *value = '.';
                        digit
                    }
                    None => break,
                };
                num += digit * mul;
                mul *= 10;
                start = point;
            }
            while let Some(point) = end.right() {
                let value = match schematic.value_mut(point) {
                    Some(value) => value,
                    None => break,
                };
                let digit = match value.to_digit(10) {
                    Some(digit) => {
                        *value = '.';
                        digit
                    }
                    None => break,
                };
                num = num * 10 + digit;
                end = point;
            }
            Some(num)
        })
        .sum();
    println!("{sum}");
    Ok(())
}
