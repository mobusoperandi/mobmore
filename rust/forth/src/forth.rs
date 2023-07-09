use std::rc::Rc;

use crate::{
    builtin::Builtin, code_lexeme::CodeLexeme, lexeme::Lexeme, line::Line, stack::Stack,
    word::Word, word_or_value::WordOrValue, wordlist::Wordlist,
};

pub type Value = i32;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Default)]
pub struct Forth {
    stack: Stack,
    wordlist: Wordlist,
}

impl Forth {
    pub fn new() -> Forth {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        let line = input.parse::<Line>()?;
        self.evaluate(line)?;
        Ok(())
    }

    fn evaluate(&mut self, line: Line) -> Result<(), Error> {
        line.into_iter().try_for_each(|lexeme| {
            match lexeme {
                Lexeme::WordOrValue(WordOrValue::Value(value)) => {
                    self.stack.push(value);
                }
                Lexeme::WordOrValue(WordOrValue::Word(word)) => {
                    self.apply_word(word)?;
                }
                Lexeme::ColonDefinition(colon_definition) => {
                    self.wordlist.insert(colon_definition);
                }
            };
            Ok(())
        })
    }

    fn apply_word(&mut self, word: Word) -> Result<(), Error> {
        if let Some(code) = self.wordlist.get(&word) {
            return self.evaluate_code(code);
        }

        if let Some(builtin) = Builtin::from_word(&word) {
            builtin.apply(&mut self.stack)?;
            return Ok(());
        }

        Err(Error::UnknownWord)
    }

    fn evaluate_code(&mut self, code: Rc<Vec<CodeLexeme>>) -> Result<(), Error> {
        code.iter().try_for_each(|code_lexeme| match code_lexeme {
            &CodeLexeme::Value(value) => {
                self.stack.push(value);
                Ok(())
            }
            CodeLexeme::Word(word) => self.apply_word(word.clone()),
            CodeLexeme::RedefinedWord(code) => self.evaluate_code(code.clone()),
        })?;
        Ok(())
    }
}
