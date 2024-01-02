pub use Node::*;

use anyhow::{bail, Error, Result};
use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Node {
    Space,
    RoundedRock,
    CubeShapedRock,
}

impl TryFrom<char> for Node {
    type Error = Error;

    fn try_from(character: char) -> Result<Self> {
        Ok(match character {
            '.' => Space,
            'O' => RoundedRock,
            '#' => CubeShapedRock,
            _ => bail!("Cannot convert character '{character}' to a Node"),
        })
    }
}

impl From<Node> for char {
    fn from(node: Node) -> Self {
        match node {
            Space => '.',
            RoundedRock => 'O',
            CubeShapedRock => '#',
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char((*self).into())
    }
}
