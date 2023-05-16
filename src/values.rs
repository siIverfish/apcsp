use crate::basic_types::*;

// todo: breakup large functions into smaller bits

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Operation(Operation),
    String(String),
    Operator(Operator),
    Group(Vec<Value>),
    ProcedureCall { name: String, args: Vec<Value> },
}

// impl Value {
//     fn unwrap_tree(&self) {
//         let Value::G
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operation {
    operator: Operator,
    args: Vec<Value>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Operator {
    Comma,
    Not, And, Or,
    Eq, Neq, Gte, Lte, Lt, Gt,
    Add, Sub,
    Mul, Div, Mod,
    Pow,
    OpenParen, CloseParen,
}

impl Operator {
    const STR_TO_OPERATOR: [(&'static str, Operator); 18] = [
        (",",   Operator::Comma),
        ("NOT", Operator::Not),
        ("AND", Operator::And),
        ("OR",  Operator::Or),
        ("=",   Operator::Eq),
        ("!=",  Operator::Neq),
        ("<=",  Operator::Lte),
        (">=",  Operator::Gte),
        ("<",   Operator::Lt),
        (">",   Operator::Gt),
        ("+",   Operator::Add),
        ("-",   Operator::Sub),
        ("*",   Operator::Mul),
        ("/",   Operator::Div),
        ("%",   Operator::Mod),
        ("^",   Operator::Pow),
        ("(",   Operator::OpenParen),
        (")",   Operator::CloseParen),
    ];

}

// todo delegate the precedence vec to something else
pub fn parse_value(value: &str) -> Value {
    use Operator::*;

    let precedence: [Vec<Operator>; 5] = [
        vec![Pow],
        vec![Mul, Div],
        vec![Add, Sub],
        vec![Eq, Neq, Lte, Gte, Gt, Lt],
        vec![And, Not, Or],
    ];

    let value = tokenize(value);
    let value = parse_parentheses(value);

    println!("{value:?}\n\n\n\n");

    let value = parse_procedure_calls(value);
    let value = parse_operations(value, &precedence);

    value
}

// im pretty sure this code is kinda godawful
// whoops
// at least it works
fn tokenize(string: &str) -> Vec<Value> {
    let mut index: usize = 0;
    let mut tokens: Vec<Value> = Vec::new();

    'outer: while let Some(slice) = string.get(index..) {
        // operators
        for (operator_string, operator) in Operator::STR_TO_OPERATOR {
            if slice.starts_with(operator_string) {
                tokens.push(Value::Operator(operator));
                index += operator_string.len();
                continue 'outer;
            }
        }

        let mut chars = slice.chars().peekable();

        // breaking
        if chars.peek().is_none() { break };
        
        // whitespace
        if chars.peek().is_some_and(|c| c.is_whitespace()) {
            index += 1;
            continue 'outer;
        }

        // values (name, string, or number)
        let mut value: String = String::new();
        while chars.peek().is_some_and(|c: &char| c.is_ascii_alphanumeric() || c == &'_') {
            value.push(chars.next().unwrap());
            index += 1;
        }

        if !value.is_empty() {
            tokens.push(Value::String(value));
        } else {
            panic!("unexpected character: {:?}", chars.next().unwrap());
        }
    }

    tokens
}

fn parse_parentheses(values: Vec<Value>) -> Vec<Value> {
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

fn parse_procedure_calls(values: Vec<Value>) -> Vec<Value> {
    let mut new_values: Vec<Value> = Vec::with_capacity(values.len());
    let mut iter = values.into_iter().peekable();

    while let Some(value) = iter.next() {
        if let Value::String(ref name) = value {
            if let Some(&Value::Group(_)) = iter.peek() {
                // it's a procedure call!!
                let Some(Value::Group(procedure_call_args)) = iter.next()
                    else { unreachable!() };
                
                // ok, now split the tokens in the call by the commas
                // with lots of allocations :(
                let mut split_args: Vec<Vec<Value>> = vec![Vec::new()];

                for argument_token in procedure_call_args {
                    if let Value::Operator(Operator::Comma) = argument_token {
                        split_args.push(Vec::new());
                    } else {
                        split_args.last_mut().unwrap().push(argument_token);
                    }
                }

                // each argument should be recursively parsed of function calls
                // and turned into a group
                let args = split_args
                    .into_iter()
                    .map(parse_procedure_calls)
                    .map(Value::Group)
                    .collect::<Vec<_>>();

                // voila!
                new_values.push(Value::ProcedureCall { name: name.to_string(), args })
            } else {
                new_values.push(value);
            }
        } else {
            // it's not a procedure call..
            new_values.push(value);
        }
    }

    new_values
}


fn parse_operations(values: Vec<Value>, precedence: &[Vec<Operator>; 5]) -> Value {
    let values: Vec<Value> = values
        .into_iter()
        .map(|value| {
            if let Value::Group(more_values) = value {
                parse_operations(more_values, precedence)
            } else if let Value::ProcedureCall { name, args } = value {
                Value::ProcedureCall { name, args: args
                    .into_iter()
                    .map(|value| {
                        if let Value::Group(values) = value {
                            parse_operations(values, precedence)
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
        let right_side = parse_operations(values[operator_index+1..].to_vec(), precedence);
    
        Value::Operation(Operation { operator: found_operator.unwrap(), args: [right_side].to_vec()})
    } else {
        // normal execution 
        let left_side  = parse_operations(values[  ..operator_index].to_vec(), precedence);
        let right_side = parse_operations(values[operator_index+1..].to_vec(), precedence);
    
        Value::Operation(Operation { operator: found_operator.unwrap(), args: [left_side, right_side].to_vec()})
    }
}
