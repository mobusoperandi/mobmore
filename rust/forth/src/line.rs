use itertools::Itertools;

use crate::{lexeme::Lexeme, Error};

#[derive(Debug, Clone)]
pub(crate) struct Line(Vec<Lexeme>);

impl IntoIterator for Line {
    type Item = Lexeme;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::str::FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .chars()
            .batching(|characters| {
                let mut in_colon_definition = false;
                let mut lexeme = String::new();
                loop {
                    match characters.next() {
                        None => {
                            return if in_colon_definition {
                                Some(Err(Error::InvalidWord))
                            } else if lexeme.is_empty() {
                                None
                            } else {
                                Some(Ok(lexeme))
                            }
                        }
                        Some(character) => {
                            match character {
                                ':' => {
                                    if in_colon_definition {
                                        return Some(Err(Error::InvalidWord));
                                    }
                                    in_colon_definition = true;
                                }
                                ';' => {
                                    if !in_colon_definition {
                                        return Some(Err(Error::InvalidWord));
                                    }
                                    in_colon_definition = false;
                                }
                                ' ' if !in_colon_definition && !lexeme.is_empty() => {
                                    return Some(Ok(lexeme));
                                }
                                _ => {}
                            }

                            lexeme.push(character)
                        }
                    };
                }
            })
            .map(|result| result.and_then(|lexeme| lexeme.parse::<Lexeme>()))
            .try_collect()?;

        Ok(Self(line))
    }
}
