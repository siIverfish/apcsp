use crate::basic_types::*;
use crate::basics::BasicStatement;

// todo: breakup large functions into smaller bits
// todo: use more impls

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Operation {
        operator: Operator,
        args: Vec<Value>,
    },
    Raw(String),
    Operator(Operator),
    Group(Vec<Value>),
    ProcedureCall { name: String, args: Vec<Value> },

    I128(i128),
    Name(String),
    String(String),
}

pub fn to_values(statements: Vec<BasicStatement>) -> Vec<BasicStatement> {
    statements
        .into_iter()
        .map(|statement| {
            match statement {
                BasicStatement::Assignment { name, value } => {
                    BasicStatement::Assignment { name: name.parse(), value: value.parse() }
                },

                BasicStatement::Procedure { name, args, code, value } => {
                    BasicStatement::Procedure { name: name.parse(), args: args.parse(), code: to_values(code), value: value.parse() }
                },

                BasicStatement::Return { value } => {
                    BasicStatement::Return { value: value.parse() }
                }

                BasicStatement::Control { kind, values, block } => {
                    BasicStatement::Control { kind, values: values.into_iter().map(|v| Value::parse(&v)).collect::<Vec<_>>(), block: to_values(block) }
                }
            }
        })
        .collect::<Vec<_>>()
}

impl Value {
    pub fn parse(&self) -> Value {
        let Value::Raw(string) = self
            else { panic!("parse_raw_value must get a Value::Raw")};
    
        parse_value(&string)
    }

    // should? always return Value::String
    // wish i could explicitly type that
    // theres probably some way to
    fn apply_to_all(&self, func: &impl Fn(&Value) -> Value) -> Value {
        func(&match self {
            &Value::Operation { ref args, ref operator, .. }  => {
                Value::Operation { 
                    operator: operator.clone(),
                    args: args
                        .iter()
                        .map(|value| Value::apply_to_all(value, func))
                        .map(|value| func(&value))
                        .collect::<Vec<_>>()
                    } 
            }

            &Value::Group(ref values) => {
                Value::Group(
                    values
                        .iter()
                        .map(|value| Value::apply_to_all(value, func))
                        .map(|value| func(&value))
                        .collect::<Vec<_>>()
                )
            }

            &Value::ProcedureCall { ref name, ref args } => {
                Value::ProcedureCall { name: 
                    name.clone(), 
                    args: args
                        .iter()
                        .map(|value| Value::apply_to_all(value, func))
                        .map(|value| func(&value))
                        .collect::<Vec<_>>()
                 }
            }

            _ => self.clone()
        })
    }

    fn apply_to_all_strings(&self, func: &impl Fn(&Value) -> Value) -> Value {
        self.apply_to_all(&|value| {
            if let Value::String(_) = value {
                func(value)
            } else {
                value.clone()
            }
        })
    }
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
    OpenBracket, CloseBracket
}

impl Operator {
    const STR_TO_OPERATOR: [(&'static str, Operator); 20] = [
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
        ("MOD", Operator::Mod),
        ("^",   Operator::Pow),
        ("(",   Operator::OpenParen),
        (")",   Operator::CloseParen),
        ("[",   Operator::OpenBracket),
        ("]",   Operator::CloseBracket),
    ];

}

pub fn parse_value(value: &str) -> Value {
    let value = tokenize(value);

    parse_tokenized_value(value)
}

fn parse_tokenized_value(value: Vec<Value>) -> Value {
    if let Some(value) = parse_list(&value) { return value; };
    
    let value = parse_parentheses(value);
    let value = parse_procedure_calls(value);
    let value      = parse_operations(value);
    let value      = parse_string_types(value);
    let value      = flatten_single_vecs(value);

    value
}

fn parse_list(values: &Vec<Value>) -> Option<Value> {
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
    
            Value::Group(valuess
                .into_iter()
                .map(|values| parse_tokenized_value(values))
                .collect::<Vec<_>>())
        })
    } else {
        None
    }
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

fn parse_operations(values: Vec<Value>) -> Value {
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

fn flatten_single_vecs(value: Value) -> Value {
    value.apply_to_all(&|value| {
        match value {
            &Value::Group(ref values) => {
                if values.len() == 1 {
                    values[0].clone()
                } else {
                    value.clone()
                }
            },

            _ => value.clone(),
        }
    })
}

fn parse_string_types(value: Value) -> Value {
    value.apply_to_all_strings(&|value| {
        let Value::String(value) = value 
            else { unreachable!("apply_to_all should only give Value::Strings or other non-Value-containing types."); };

        if let Ok(int_value) = str::parse::<i128>(value) {
            Value::I128(int_value)
        } else if let Some(string_value) = value.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            Value::String(string_value.into())
        } else if value.chars().all(|c| c.is_alphabetic() || c == '_' ) {
            Value::Name(value.into())
        } else {
            panic!("could not understand value: {value:?}");
        }
    })
}