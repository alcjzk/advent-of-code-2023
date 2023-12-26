use anyhow::{Error, Result};
use itertools::Itertools;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct History(Vec<Sequence>);

impl History {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push(&mut self, sequence: Sequence) {
        self.0.push(sequence)
    }
    fn predict_next(&mut self) {
        let mut value = 0;

        self.0.iter_mut().rev().for_each(|sequence| {
            value += sequence.last();
            sequence.push(value)
        });
    }
    fn predict_prev(&mut self) {
        let mut value = 0;

        self.0.iter_mut().rev().for_each(|sequence| {
            value = sequence.first() - value;
            sequence.push_front(value);
        })
    }
    fn first(&self) -> &Sequence {
        self.0.first().expect("Expected first sequence in history")
    }
}

impl From<Sequence> for History {
    fn from(mut sequence: Sequence) -> Self {
        let mut history = History::new();

        while !sequence.is_all_zero() {
            let next = Sequence::from_differences(&sequence);
            history.push(sequence);
            sequence = next;
        }
        history.push(sequence);
        history
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Sequence(Vec<isize>);

impl Sequence {
    fn is_all_zero(&self) -> bool {
        !self.0.iter().copied().any(|n| n != 0)
    }
    fn from_differences(other: &Sequence) -> Self {
        Self(
            other
                .0
                .iter()
                .copied()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect(),
        )
    }
    fn last(&self) -> isize {
        self.0
            .iter()
            .copied()
            .last()
            .expect("Expected non empty sequence")
    }
    fn first(&self) -> isize {
        *self.0.first().expect("Expected no empty sequence")
    }
    fn push(&mut self, value: isize) {
        self.0.push(value)
    }
    fn push_front(&mut self, value: isize) {
        self.0.insert(0, value)
    }
}

impl From<Vec<isize>> for Sequence {
    fn from(value: Vec<isize>) -> Self {
        Self(value)
    }
}

fn part_one(histories: &mut [History]) -> Result<String> {
    histories
        .iter_mut()
        .for_each(|history| history.predict_next());
    let sum: isize = histories.iter().map(|history| history.first().last()).sum();
    Ok(sum.to_string())
}

fn part_two(histories: &mut [History]) -> Result<String> {
    histories
        .iter_mut()
        .for_each(|history| history.predict_prev());
    let sum: isize = histories
        .iter()
        .map(|history| history.first().first())
        .sum();
    Ok(sum.to_string())
}

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("input")?;

    let mut histories: Vec<_> = BufReader::new(file)
        .lines()
        .map(|maybe_line| {
            let line = maybe_line?;

            let values = line
                .split_ascii_whitespace()
                .map(|value| Ok(value.parse::<isize>()?))
                .collect::<Result<Vec<isize>, Error>>()?;

            Ok(History::from(Sequence::from(values)))
        })
        .collect::<Result<_, Error>>()?;

    println!("part one answer: {}", part_one(&mut histories)?);
    println!("part two answer: {}", part_two(&mut histories)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_from_differences() {
        assert_eq!(
            Sequence::from_differences(&Sequence::from(vec![0, 2, 3])),
            Sequence::from(vec![2, 1])
        );
    }

    #[test]
    fn sequence_is_all_zero() {
        assert!(Sequence::from(vec![0, 0, 0]).is_all_zero());
        assert!(Sequence::from(vec![0]).is_all_zero());
        assert!(!Sequence::from(vec![2]).is_all_zero());
        assert!(!Sequence::from(vec![2, 3]).is_all_zero());
    }
}
