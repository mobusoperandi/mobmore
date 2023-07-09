use std::str::FromStr;

use crate::{colon_definition::ColonDefinition, word_or_value::WordOrValue, Error};

#[derive(Debug, Clone)]
pub(crate) enum Lexeme {
    WordOrValue(WordOrValue),
    ColonDefinition(ColonDefinition),
}

impl FromStr for Lexeme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [':', ' ', word_and_sequence @ .., ' ', ';'] =
            s.chars().collect::<Vec<char>>().as_slice()
        {
            let word_and_sequence = String::from_iter(word_and_sequence);
            let Some((word, sequence)) = word_and_sequence.split_once(' ') else {
                return Err(Error::InvalidWord);
            };

            let sequence = sequence.parse()?;
            let word = word.parse()?;

            return Ok(Self::ColonDefinition(ColonDefinition::new(word, sequence)));
        }

        Ok(Self::WordOrValue(s.parse()?))
    }
}
