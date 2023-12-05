use anyhow::{anyhow, Result, Error};

#[derive(Debug)]
pub struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    pub fn points(&self) -> usize {
        let count = self.numbers.iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count();
        if count > 0 {
            return 2usize.pow((count - 1) as u32);
        }
        0
    }
}

impl TryFrom<String> for Card {
    type Error = Error;

    fn try_from(line: String) -> Result<Self> {
        fn _numbers(string: &str) -> Result<Vec<usize>> {
            Ok(string.split_ascii_whitespace()
                .into_iter()
                .map(|number|Ok(number.parse::<usize>()?))
                .collect::<Result<_>>()?)
        }

        let values = line.split(':')
            .nth(1)
            .ok_or(anyhow!("Invalid card format '{line}'"))?;

        let mut split = values.split('|');
        let winning_numbers = _numbers(split.next()
            .ok_or(anyhow!("Invalid card format '{line}'"))?)?;
        let numbers = _numbers(split.next()
            .ok_or(anyhow!("Invalid card format '{line}'"))?)?;
        
        Ok(Card {
            winning_numbers,
            numbers,
        })
    }
}
