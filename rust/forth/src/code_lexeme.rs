use std::rc::Rc;

use crate::{word::Word, Value};

#[derive(Debug, Clone)]
pub(crate) enum CodeLexeme {
    Value(Value),
    Word(Word),
    RedefinedWord(Rc<Vec<CodeLexeme>>),
}
