use super::*;

pub(crate) fn parse_parentheses(values: Vec<Value>) -> Vec<Value> {
    fn parse_parentheses_inner(values: &mut impl Iterator<Item = Value>) -> Vec<Value> {
        let mut new_values: Vec<Value> = Vec::new(); // todo with capacity

        while let Some(value) = values.next() {
            match value {
                Value::Operator(Operator::CloseParen) => return new_values,
                Value::Operator(Operator::OpenParen)  => new_values.push(Value::Group(parse_parentheses_inner(values))),
                _ => new_values.push(value),
            }
        }

        new_values
    }

    let mut values_iter = values.into_iter();
    parse_parentheses_inner(&mut values_iter)
}