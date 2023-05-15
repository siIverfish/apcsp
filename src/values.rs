
#[derive(Debug)]
pub enum Value {
    Operation(Operation),
    String(String),
    Operator(Operator),
    Group(Vec<Value>),
}

#[derive(Debug)]
pub struct Operation {
    operator: Operator,
    args: (Option<Box<Value>>, Option<Box<Value>>),
}

#[derive(Debug)]
pub enum Operator {
    Not, And, Or,
    Eq, Neq, Gte, Lte, Lt, Gt,
    Add, Sub,
    Mul, Div, Mod,
    Pow,
    OpenParen, CloseParen,
}

impl Operator {
    const OPERATOR_CHARS: &'static str = "!=<>+-*/^";

    const STR_TO_OPERATOR: [(&'static str, Operator); 17] = [
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

pub fn parse_value(value: &str) -> Vec<Value> { //Operation {
    use Operator::*;

    let precedence: [Vec<Operator>; 5] = [
        vec![And, Not, Or],
        vec![Eq, Neq, Lte, Gte, Gt, Lt],
        vec![Add, Sub],
        vec![Mul, Div],
        vec![Pow],
    ];

    let value = tokenize(value);
    let value = parse_parentheses(value);

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
        tokens.push(Value::String(value));
    }

    tokens
}

fn parse_parentheses(values: Vec<Value>) -> Vec<Value> {
    fn parse_parentheses_inner(mut values: &mut impl Iterator<Item = Value>) -> Vec<Value> {
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

fn parse_operations(values: Vec<Value>, precedence: &[Vec<Operator>; 5]) -> Operation {
    

    // for operator_set in precedence {
    //     let operator_indeces = operator_set 
    //         .map(|operator| )
    // }

    todo!();
}