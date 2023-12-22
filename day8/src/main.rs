use anyhow::{Result, Error, anyhow};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

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
        let key = split.next().ok_or(anyhow!("Missing key on line '{line}'"))?.try_into()?;
        let _ = split.next().ok_or(anyhow!("Invalid format on line '{line}'"))?;
        let left = split.next()
            .ok_or(anyhow!("Missing left element on line '{line}'"))?
            .get(1..4)
            .ok_or(anyhow!("Invalid format on line '{line}'"))?
            .try_into()?;
        let right = split.next()
            .ok_or(anyhow!("Missing right element on line '{line}'"))?
            .get(0..3)
            .ok_or(anyhow!("Invalid format on line '{line}'"))?
            .try_into()?;
        Ok(Node {
            key,
            left,
            right,
        })
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
}

impl FromIterator<Node> for Map {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

fn part_one(instructions: &str, map: &Map) -> Result<String> {
    let mut current = map.find(|node|node.key == Element(['A','A','A'])).ok_or(anyhow!("Missing starting point in map"))?;
    let mut steps = 0;
        
    loop {
        let element = instructions.chars().find_map(|c|{
            current = match c {
                'R' => map.find(|node|node.key == current.right).unwrap(),
                'L' => map.find(|node|node.key == current.left).unwrap(),
                _ => panic!("Unexpected instruction '{c}'"),
            };
            steps += 1;
            if current.key == Element(['Z','Z','Z']) {
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

fn part_two(instructions: &str, map: &Map) -> Result<String> {
    // find all nodes ending with A
    todo!()
}

fn main() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("test")?;

    let mut lines = BufReader::new(file).lines();

    let instructions = lines.next().ok_or(anyhow!("Missing instructions from input"))??;
    let _ = lines.next().unwrap();

    let map: Map = lines.map(|maybe_line|{
        let line = maybe_line?;
        Ok(Node::try_from(line.as_str())?)
    })
    .collect::<Result<_>>()?;

    let part_one_answer = part_one(&instructions, &map)?;
    let part_two_answer = part_two(&instructions, &map)?;

    println!("part one: {part_one_answer}");
    println!("part two: {part_two_answer}");

    Ok(())
}
