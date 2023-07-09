use crate::{word::Word, Error, Value};

#[derive(Debug, Clone)]
pub(crate) enum WordOrValue {
    Word(Word),
    Value(Value),
}

impl std::str::FromStr for WordOrValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(word) = Word::from_str(s) {
            return Ok(Self::Word(word));
        }

        if let Ok(value) = Value::from_str(s) {
            return Ok(Self::Value(value));
        }

        Err(Error::InvalidWord)
    }
}
