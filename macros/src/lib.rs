#[macro_export]
macro_rules! char_enum {
    {
        $(#[derive($($x:ident),+)])?
        $visibility:vis $name:ident {
            $($variant:ident => $character:literal$(,)?)+
        }
    } => {
        $(#[derive($($x),+)])?
        $visibility enum $name {
            $($variant),+
        }

        impl TryFrom<char> for $name {
            type Error = anyhow::Error;

            fn try_from(character: char) -> anyhow::Result<Self> {
                Ok(match character {
                    $($character => $name::$variant),+,
                    _ => anyhow::bail!("Cannot convert character '{character}' to a {}", stringify!($name)),
                })
            }
        }

        impl From<$name> for char {
            fn from(value: $name) -> char {
                match value {
                    $($name::$variant => $character),+,
                }
            }
        }

        impl From<&$name> for char {
            fn from(value: &$name) -> char {
                match value {
                    $(&$name::$variant => $character),+,
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Write::write_char(f, self.into())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    char_enum! {
        #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
        pub Node {
            First => '1',
            Second => '2',
        }
    }

    #[test]
    fn display() {
        assert_eq!(Node::First.to_string(), "1");
    }
}
