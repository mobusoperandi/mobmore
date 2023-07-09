use std::{collections::BTreeMap, rc::Rc};

use crate::{
    code_lexeme::CodeLexeme, colon_definition::ColonDefinition, word::Word,
    word_or_value::WordOrValue,
};

#[derive(Default)]
pub(crate) struct Wordlist(BTreeMap<Word, Rc<Vec<CodeLexeme>>>);

impl Wordlist {
    pub(crate) fn insert(&mut self, colon_definition: ColonDefinition) {
        let ColonDefinition { word, sequence } = colon_definition;

        let code = sequence
            .into_iter()
            .map(|word_or_value| match word_or_value {
                WordOrValue::Value(value) => CodeLexeme::Value(value),
                WordOrValue::Word(word) => {
                    if let Some(code) = self.get(&word) {
                        CodeLexeme::RedefinedWord(code)
                    } else {
                        CodeLexeme::Word(word)
                    }
                }
            })
            .collect();

        self.0.insert(word, Rc::new(code));
    }

    pub(crate) fn get(&self, word: &Word) -> Option<Rc<Vec<CodeLexeme>>> {
        self.0.get(word).cloned()
    }
}
