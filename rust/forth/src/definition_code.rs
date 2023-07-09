use std::rc::Rc;

use crate::{colon_definition::ColonDefinition, word_or_value::WordOrValue};

use super::{code_lexeme::CodeLexeme, wordlist::Wordlist};

#[derive(Debug, Clone)]
pub(crate) struct DefinitionCode(Vec<CodeLexeme>);

impl DefinitionCode {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &CodeLexeme> {
        self.0.iter()
    }
}
