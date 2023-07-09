use crate::{Error, Value};

#[derive(Debug, Default)]
pub(crate) struct Stack(Vec<Value>);

impl Stack {
    pub(crate) fn try_pop(&mut self) -> Result<Value, Error> {
        self.0.pop().ok_or(Error::StackUnderflow)
    }

    pub(crate) fn push(&mut self, value: Value) {
        self.0.push(value)
    }

    pub(crate) fn as_slice(&self) -> &[Value] {
        self.0.as_slice()
    }
}
