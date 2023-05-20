use crate::values::operator::Operator;
use crate::values::Value;

pub(crate) fn parse_operations(values: Vec<Value>) -> Value {
    use Operator::*;

    let precedence: [Vec<Operator>; 5] = [
        vec![Pow],
        vec![Mul, Div],
        vec![Add, Sub],
        vec![Eq, Neq, Lte, Gte, Gt, Lt],
        vec![And, Not, Or],
    ];

    parse_operations_inner(values, &precedence)
}

fn parse_operations_inner(values: Vec<Value>, precedence: &[Vec<Operator>; 5]) -> Value {
    let values: Vec<Value> = values
        .into_iter()
        .map(|value| {
            if let Value::Group(more_values) = value {
                parse_operations_inner(more_values, precedence)
            } else if let Value::ProcedureCall { name, args } = value {
                Value::ProcedureCall { name, args: args
                    .into_iter()
                    .map(|value| {
                        if let Value::Group(values) = value {
                            parse_operations_inner(values, precedence)
                        } else {
                            value
                        }
                    })
                    .collect::<Vec<_>>()
                }
            } else {
                value
            }
        })
        .collect();


    // find the index of the first operator, sorted by:
    // 1. precedence in `precedence` vec
    // 2. operator list position
    let mut operator_index: Option<usize> = None;
    let mut found_operator: Option<Operator> = None;
    for operator_set in precedence {
        for (current_index, value) in values.iter().enumerate() {
            for operator in operator_set {
                if let Value::Operator(current_operator) = value {
                    if current_operator == operator {
                        operator_index = Some(current_index);
                        found_operator = Some(operator.clone());
                        break;
                    }
                }
            }
        }
    }

    let operator_index = match operator_index {
        Some(index) => index,
        None               => return Value::Group(values),
    };


    if let Some(Operator::Not) = found_operator {
        // handle special case: NOT, which takes only 1 argument.
        // there's certainly a better solution for this.
        // i should probably make `args` a vector
        let right_side = parse_operations_inner(values[operator_index+1..].to_vec(), precedence);
    
        Value::Operation { operator: found_operator.unwrap(), args: [right_side].to_vec()}
    } else {
        // normal execution 
        let left_side  = parse_operations_inner(values[  ..operator_index].to_vec(), precedence);
        let right_side = parse_operations_inner(values[operator_index+1..].to_vec(), precedence);
    
        Value::Operation { operator: found_operator.unwrap(), args: [left_side, right_side].to_vec()}
    }
}