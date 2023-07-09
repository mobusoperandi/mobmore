use crate::{word::Word, Error};

use super::stack::Stack;

#[derive(Debug, Clone)]
pub(crate) enum Builtin {
    Add,
    Sub,
    Div,
    Mul,
    Dup,
    Drop,
    Swap,
    Over,
}

impl Builtin {
    pub(crate) fn apply(&self, value_stack: &mut Stack) -> Result<(), Error> {
        let implementation = match self {
            Builtin::Add => add,
            Builtin::Sub => sub,
            Builtin::Div => div,
            Builtin::Mul => mul,
            Builtin::Dup => dup,
            Builtin::Drop => drop_,
            Builtin::Swap => swap,
            Builtin::Over => over,
        };
        implementation(value_stack)
    }

    pub(crate) fn from_word(word: &Word) -> Option<Self> {
        match word.as_str() {
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "/" => Some(Self::Div),
            "*" => Some(Self::Mul),
            "DUP" => Some(Self::Dup),
            "DROP" => Some(Self::Drop),
            "SWAP" => Some(Self::Swap),
            "OVER" => Some(Self::Over),
            _ => None,
        }
    }
}

pub(super) fn add(stack: &mut Stack) -> Result<(), Error> {
    let left = stack.try_pop()?;
    let right = stack.try_pop()?;
    stack.push(left + right);
    Ok(())
}

pub(super) fn sub(stack: &mut Stack) -> Result<(), Error> {
    let left = stack.try_pop()?;
    let right = stack.try_pop()?;
    stack.push(right - left);
    Ok(())
}

pub(super) fn div(stack: &mut Stack) -> Result<(), Error> {
    let left = stack.try_pop()?;
    let right = stack.try_pop()?;
    let result = right.checked_div(left).ok_or(Error::DivisionByZero)?;

    stack.push(result);
    Ok(())
}

pub(super) fn mul(stack: &mut Stack) -> Result<(), Error> {
    let left = stack.try_pop()?;
    let right = stack.try_pop()?;
    stack.push(left * right);
    Ok(())
}

pub(super) fn dup(stack: &mut Stack) -> Result<(), Error> {
    let value = stack.try_pop()?;
    stack.push(value);
    stack.push(value);
    Ok(())
}

pub(super) fn drop_(stack: &mut Stack) -> Result<(), Error> {
    stack.try_pop()?;
    Ok(())
}

pub(super) fn swap(stack: &mut Stack) -> Result<(), Error> {
    let left = stack.try_pop()?;
    let right = stack.try_pop()?;
    stack.push(left);
    stack.push(right);
    Ok(())
}

pub(super) fn over(stack: &mut Stack) -> Result<(), Error> {
    let left = stack.try_pop()?;
    let right = stack.try_pop()?;
    stack.push(right);
    stack.push(left);
    stack.push(right);
    Ok(())
}
