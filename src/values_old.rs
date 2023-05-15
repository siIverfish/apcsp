
// trait IsValidRaw {
//     fn is_valid_raw(&self) -> bool;
// }

// impl IsValidRaw for char {
//     fn is_valid_raw(&self) -> bool {
//         self.is_ascii_alphabetic() || self == &'_' || self.is_alphanumeric()
//     }
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum Operator {
//     // reverse precedence order

//     Not,
//     And,
//     Or,

//     Eq,
//     Neq,
//     Gte,
//     Lte,
//     Lt,
//     Gt,

//     Add,
//     Sub,

//     Mul,
//     Div,
//     Mod,
    
//     Pow,

//     OpenParen,
//     CloseParen,
// }

// impl Operator {
//     const ALL_OPERATOR_CHARS: &'static str = "()^*/+-<>=%";

//     fn is_operator(c: char) -> bool {
//         Operator::ALL_OPERATOR_CHARS.contains(c)
//     }

//     fn from_string(s: String) -> Option<Operator> {
//         Some(match s.trim() {
//             "AND" => Operator::And,
//             "OR"  => Operator::Or,
//             "NOT" => Operator::Not,
//             "!="  => Operator::Neq,
//             ">="  => Operator::Gte,
//             "<="  => Operator::Gte,
//             _ => None?,
//         })
//     }
// }

// impl From<char> for Operator {
//     fn from(c: char) -> Operator {
//         match c {
//             '(' => Operator::OpenParen,
//             ')' => Operator::CloseParen,
//             '^' => Operator::Pow,
//             '*' => Operator::Mul,
//             '/' => Operator::Div,
//             '+' => Operator::Add,
//             '-' => Operator::Sub,
//             '>' => Operator::Gt,
//             '<' => Operator::Lt,
//             '=' => Operator::Eq,
//             '%' => Operator::Mod,
//             _   => panic!("invalid operator passed to From<char> for Operator"),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum ValueToken {
//     Operator(Option<Box<ValueToken>>, Operator, Option<Box<ValueToken>>),
//     ParenGroup(Vec<ValueToken>),
//     Raw(String),
// }

// fn initial_tokenize(string: String) -> Vec<ValueToken> {
//     let mut tokens: Vec<ValueToken> = Vec::new();
//     let mut chars = string.chars().peekable();

//     while let Some(c) = chars.next() {
//         if Operator::is_operator(c) {
//             tokens.push(ValueToken::Operator(None, Operator::from(c), None))
//         } else if c.is_valid_raw() {
//             let mut string = String::from(c);
//             while chars.peek().is_some_and(|c: &char| { c.is_valid_raw() }) {
//                 string.push(chars.next().unwrap());
//             }
//             tokens.push(ValueToken::Raw(string));
//         }
//     }

//     tokens
// }

// fn parse_parentheses(mut tokens: &mut dyn Iterator<Item = ValueToken>) -> Vec<ValueToken> {
//     let mut new_tokens: Vec<ValueToken> = Vec::new();

//     while let Some(token) = tokens.next() {
//         match token {
//             ValueToken::Operator(None, Operator::CloseParen, None) => return new_tokens,
//             ValueToken::Operator(None, Operator::OpenParen, None)  => new_tokens.push( ValueToken::ParenGroup(parse_parentheses(&mut tokens)) ),
//             _ => new_tokens.push(token),
//         }
//     }

//     new_tokens
// }

// fn parse_operators(tokens: Vec<ValueToken>) -> ValueToken {
//     use Operator::*;

//     let operator_precedence_rev: Vec<Vec<Operator>> = vec![
//         vec![Not, And, Or],
//         vec![Eq, Neq, Gte, Lte, Gt, Lt],
//         vec![Add, Sub],
//         vec![Mul, Div, Mod],
//         vec![Pow],
//     ];

//     let mut final_tree: ValueToken = ValueToken::ParenGroup(Vec::new());

//     for operators in operator_precedence_rev {
        
//     }

//     todo!();
// }

// fn parse_tokens_with_operators(tokens: Vec<ValueToken>, operators: Vec<Operator>) -> Vec<ValueToken> {
//     let mut new_tokens: Vec<ValueToken> = Vec::new();
//     let operator: ValueToken;
//     let mut tokens_iter: std::vec::IntoIter<ValueToken> = tokens.into_iter();

//     for token in tokens_iter {
//         match token {
//             ValueToken::Operator(None, operator, None) if operators.contains(&operator) => {
//                 let operator = ValueToken::Operator(
//                     Some(Box::new(ValueToken::ParenGroup(new_tokens))),
//                     operator,
//                     Some(Box::new(ValueToken::ParenGroup(tokens_iter.collect())))
//                 );
//                 break;
//             }

//             _ => new_tokens.push(token),
//         }
//     }

//     todo!();
// }

// pub fn parse_value(value: String) -> Vec<ValueToken> {
//     let tokens: Vec<ValueToken> = initial_tokenize(value);
//     let tokens: Vec<ValueToken> = parse_parentheses(&mut tokens.into_iter());

//     tokens
// }