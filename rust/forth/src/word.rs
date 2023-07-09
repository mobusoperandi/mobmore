use std::str::FromStr;

use crate::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Word(String);

impl Word {
    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Word {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (digit, alphabetic, punctuation) =
            s.chars()
                .fold((0, 0, 0), |(digit, alphabetic, punctuation), character| {
                    if character.is_ascii_digit() {
                        (digit + 1, alphabetic, punctuation)
                    } else if character.is_ascii_alphabetic() {
                        (digit, alphabetic + 1, punctuation)
                    } else if character.is_ascii_punctuation() {
                        (digit, alphabetic, punctuation + 1)
                    } else {
                        (digit, alphabetic, punctuation)
                    }
                });

        let length = s.len();

        if digit + alphabetic + punctuation < length || digit == length {
            return Err(Error::InvalidWord);
        }

        Ok(Self(s.to_uppercase()))
    }
}
