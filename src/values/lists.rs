use super::*;

pub(crate) fn parse_list(values: &Vec<Value>) -> Option<Value> {
    use Operator::*;

    if let &[Value::Operator(OpenBracket), .., Value::Operator(CloseBracket)] = values.as_slice() {
        Some({
            let mut valuess: Vec<Vec<Value>> = vec![Vec::new()];
    
            for value in values[1..values.len() - 1].iter() {
                if let &Value::Operator(Comma) = value {
                    valuess.push(Vec::new());
                } else {
                    valuess.last_mut().unwrap().push(value.clone());
                }
            }

            // support trailing commas
            // an extra comma would add an empty group to the end
            if valuess.last().unwrap().is_empty() {
                valuess.pop();
            }
    
            Value::Group(valuess
                .into_iter()
                .map(|values| parse_tokenized_value(values))
                .collect::<Vec<_>>())
        })
    } else {
        None
    }
}