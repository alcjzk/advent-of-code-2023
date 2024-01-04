use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

fn part_one<T, U>(instructions: T)
where
    T: AsRef<[U]>,
    U: AsRef<[u8]>,
{
    let sum: usize = instructions
        .as_ref()
        .iter()
        .map(|instruction| hash(instruction) as usize)
        .sum();

    println!("part one: {sum}");
}

fn part_two<T, U>(instructions: T)
where
    T: AsRef<[U]>,
    U: AsRef<[u8]>,
{
    let mut map = Map::new();

    for instruction in instructions.as_ref() {
        let mut split = instruction
            .as_ref()
            .split_inclusive(|ins| *ins == b'-' || *ins == b'=');
        let label_action = split.next().unwrap();
        let label = &label_action[..label_action.len() - 1];
        let action = label_action[label_action.len() - 1];
        if action == b'=' {
            let focal_length = split.next().unwrap()[0] - b'0';
            map.bucket_mut(label).insert(label, focal_length);
        } else {
            map.bucket_mut(label).remove(label)
        }
    }

    let mut total = 0;

    for (bucket_number, bucket) in map.inner.iter().enumerate() {
        for (n, lens) in bucket.inner.iter().enumerate() {
            let focusing_power = (1 + bucket_number) * (n + 1) * lens.value as usize;
            total += focusing_power;
        }
    }

    println!("part two: {total}");
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;

    let instructions =
        Result::<Vec<_>>::from_iter(BufReader::new(file).split(b',').map(|l| Ok(l?)))?;

    part_one(&instructions);
    part_two(&instructions);

    Ok(())
}

fn hash<T: AsRef<[u8]>>(value: T) -> u8 {
    let mut current: usize = 0;
    for byte in value.as_ref().iter().copied() {
        current += byte as usize;
        current *= 17;
        current %= 256;
    }
    current as _
}

#[derive(Debug, Clone)]
struct Entry {
    key: Vec<u8>,
    value: u8,
}

#[derive(Debug, Default, Clone)]
struct Bucket {
    inner: Vec<Entry>,
}

impl Bucket {
    fn insert<T: AsRef<[u8]>>(&mut self, key: T, value: u8) {
        let key = key.as_ref();
        if let Some(entry) = self.inner.iter_mut().find(|entry| entry.key == key) {
            entry.value = value;
        } else {
            self.inner.push(Entry {
                key: key.into(),
                value,
            });
        }
    }
    fn remove<T: AsRef<[u8]>>(&mut self, key: T) {
        let position = self
            .inner
            .iter()
            .position(|entry| entry.key.as_slice() == key.as_ref());
        if let Some(position) = position {
            let _ = self.inner.remove(position);
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    inner: Box<[Bucket]>,
}

impl Map {
    fn new() -> Self {
        Self {
            inner: vec![Bucket::default(); 256].into(),
        }
    }
    fn bucket_mut<T: AsRef<[u8]>>(&mut self, key: T) -> &mut Bucket {
        &mut self.inner[hash(key) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_examples() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }
}
