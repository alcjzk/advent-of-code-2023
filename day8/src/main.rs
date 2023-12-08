use anyhow::{Result, Error, anyhow, bail};
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
    fn find(&self, key: Element) -> Option<&Node> {
        self.0.iter().find(|node|node.key == key)
    }
    fn last(&self) -> Element {
        self.0.last().unwrap().key
    }
    fn first(&self) -> &Node {
        self.0.first().unwrap()
    }
}

impl FromIterator<Node> for Map {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
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

    let mut current = map.find(Element(['A','A','A'])).unwrap();
    let mut steps = 0;
    if current.key == Element(['Z','Z','Z']) {
        panic!()
    }
    // println!("a {}", instructions.len());
    // while current.key != Element(['D','C','R'])
    // {
    //     instructions.chars()
    //         .for_each(|ins|{
    //             println!("{current:?} -> {ins}");
    //             current = match ins {
    //                 'R' => map.find(current.right).unwrap(),
    //                 'L' => map.find(current.left).unwrap(),
    //                 _ => panic!("Unexpected instruction '{ins}'"),
    //             };
    //             steps += 1;
    //         });
    //     println!("{steps}");
    // }
    
    loop {
        // print!(": {steps}");
        let element = instructions.chars().find_map(|c|{
            println!("{current:?} -> {c}");
            current = match c {
                'R' => map.find(current.right).unwrap(),
                'L' => map.find(current.left).unwrap(),
                _ => panic!("Unexpected instruction '{c}'"),
            };
            steps += 1;
            if current.key == Element(['Z','Z','Z']) {
                println!("{current:?} {steps}");
                return Some(current.key);
            }
            None
        });
        if element.is_some() {
            break;
        }
    }

    // LLR

    println!("a{steps}");

    Ok(())
}
