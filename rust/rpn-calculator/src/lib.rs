#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .try_fold(Vec::<i32>::new(), |mut stack, item| {
            let result: i32 = if let &CalculatorInput::Value(v) = item {
                v
            } else {
                let b = stack.pop()?;
                let a = stack.pop()?;
                let op = match item {
                    CalculatorInput::Add => std::ops::Add::add,
                    CalculatorInput::Subtract => std::ops::Sub::sub,
                    CalculatorInput::Multiply => std::ops::Mul::mul,
                    CalculatorInput::Divide => std::ops::Div::div,
                    CalculatorInput::Value(_) => unreachable!(),
                };
                op(a, b)
            };
            stack.push(result);
            Some(stack)
        })
        .and_then(|stack| {
            if stack.len() == 1 {
                Some(stack[0])
            } else {
                None
            }
        })
}
