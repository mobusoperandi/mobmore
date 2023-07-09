use crate::{word::Word, word_or_value_sequence::WordOrValueSequence};

#[derive(Debug, Clone)]
pub(crate) struct ColonDefinition {
    pub(crate) word: Word,
    pub(crate) sequence: WordOrValueSequence,
}

impl ColonDefinition {
    pub(crate) fn new(word: Word, sequence: WordOrValueSequence) -> Self {
        Self { word, sequence }
    }
}
