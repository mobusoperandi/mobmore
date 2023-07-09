use itertools::Itertools;

use crate::{word_or_value::WordOrValue, Error};

#[derive(Debug, Clone)]
pub(crate) struct WordOrValueSequence(Vec<WordOrValue>);

impl IntoIterator for WordOrValueSequence {
    type Item = WordOrValue;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::str::FromStr for WordOrValueSequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(' ')
                .filter(|part| !part.is_empty())
                .map(|part| part.parse())
                .try_collect()?,
        ))
    }
}
