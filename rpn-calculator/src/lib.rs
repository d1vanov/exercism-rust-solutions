#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for i in inputs {
        match i {
            CalculatorInput::Add => {
                let first = stack.pop()?;
                let second = stack.pop()?;
                stack.push(second + first);
            },
            CalculatorInput::Subtract => {
                let first = stack.pop()?;
                let second = stack.pop()?;
                stack.push(second - first);
            },
            CalculatorInput::Multiply => {
                let first = stack.pop()?;
                let second = stack.pop()?;
                stack.push(second * first);
            },
            CalculatorInput::Divide => {
                let first = stack.pop()?;
                let second = stack.pop()?;
                stack.push(second / first);
            },
            CalculatorInput::Value(v) => {
                stack.push(*v);
            }
        }
    }

    if stack.len() == 1 {
        return Some(stack[0]);
    }

    return None;
}
