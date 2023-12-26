use anyhow::{anyhow, bail, Error, Result};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use Instruction::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Element([char; 3]);

impl TryFrom<&str> for Element {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let mut chars = value.chars();
        Ok(Self([
            chars.next().ok_or(anyhow!("Invalid element '{value}'"))?,
            chars.next().ok_or(anyhow!("Invalid element '{value}'"))?,
            chars.next().ok_or(anyhow!("Invalid element '{value}'"))?,
        ]))
    }
}

impl Element {
    fn last(&self) -> char {
        self.0[2]
    }
}

#[derive(Debug)]
struct Node {
    key: Element,
    left: Element,
    right: Element,
}

impl TryFrom<&str> for Node {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self> {
        let mut split = line.split_ascii_whitespace();
        let key = split
            .next()
            .ok_or(anyhow!("Missing key on line '{line}'"))?
            .try_into()?;
        let _ = split
            .next()
            .ok_or(anyhow!("Invalid format on line '{line}'"))?;
        let left = split
            .next()
            .ok_or(anyhow!("Missing left element on line '{line}'"))?
            .get(1..4)
            .ok_or(anyhow!("Invalid format on line '{line}'"))?
            .try_into()?;
        let right = split
            .next()
            .ok_or(anyhow!("Missing right element on line '{line}'"))?
            .get(0..3)
            .ok_or(anyhow!("Invalid format on line '{line}'"))?
            .try_into()?;
        Ok(Node { key, left, right })
    }
}

#[derive(Debug)]
struct Map(Vec<Node>);

impl Map {
    fn find<P>(&self, predicate: P) -> Option<&Node>
    where
        P: FnMut(&&Node) -> bool,
    {
        self.0.iter().find(predicate)
    }
    fn filter<P>(&self, predicate: P) -> impl Iterator<Item = &Node>
    where
        P: FnMut(&&Node) -> bool,
    {
        self.0.iter().filter(predicate)
    }
}

impl FromIterator<Node> for Map {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

fn least_common_multiple(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = least_common_multiple(&nums[1..]);
    a * b / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn part_one(instructions: &Instructions, map: &Map) -> Result<String> {
    let mut current = map
        .find(|node| node.key == Element(['A', 'A', 'A']))
        .ok_or(anyhow!("Missing starting point in map"))?;
    let mut steps = 0;

    loop {
        let element = instructions.0.iter().find_map(|c| {
            current = match c {
                Left => map.find(|node| node.key == current.left).unwrap(),
                Right => map.find(|node| node.key == current.right).unwrap(),
            };
            steps += 1;
            if current.key == Element(['Z', 'Z', 'Z']) {
                return Some(current.key);
            }
            None
        });
        if element.is_some() {
            break;
        }
    }
    Ok(steps.to_string())
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instructions(Vec<Instruction>);

impl TryFrom<&str> for Instructions {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self> {
        Ok(Self(
            line.chars()
                .map(|c| {
                    Ok(match c {
                        'L' => Left,
                        'R' => Right,
                        _ => bail!("Unknown instruction `{c}`"),
                    })
                })
                .collect::<Result<_>>()?,
        ))
    }
}

fn part_two(instructions: &Instructions, map: &Map) -> Result<String> {
    let current_nodes: Vec<_> = map.filter(|node| node.key.last() == 'A').collect();

    let pattern_lengths: Vec<usize> = current_nodes
        .into_iter()
        .map(|mut current| {
            let mut steps = 0;

            loop {
                let element = instructions.0.iter().find_map(|c| {
                    current = match c {
                        Right => map.find(|node| node.key == current.right).unwrap(),
                        Left => map.find(|node| node.key == current.left).unwrap(),
                    };
                    steps += 1;
                    if current.key.last() == 'Z' {
                        return Some(current.key);
                    }
                    None
                });
                if element.is_some() {
                    break;
                }
            }
            steps
        })
        .collect();

    let lcm = least_common_multiple(&pattern_lengths);

    Ok(lcm.to_string())
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("test")?;

    let mut lines = BufReader::new(file).lines();

    let instructions = Instructions::try_from(
        lines
            .next()
            .ok_or(anyhow!("Missing instructions from input"))??
            .as_str(),
    )?;

    let _ = lines.next().unwrap();

    let map: Map = lines
        .map(|maybe_line| {
            let line = maybe_line?;
            Node::try_from(line.as_str())
        })
        .collect::<Result<_>>()?;

    let part_one_answer = part_one(&instructions, &map)?;
    let part_two_answer = part_two(&instructions, &map)?;

    println!("part one: {part_one_answer}");
    println!("part two: {part_two_answer}");

    Ok(())
}
