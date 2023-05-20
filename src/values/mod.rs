mod operator;
mod operations;
mod tidying;
mod procedures;
mod parens;
mod lists;

use crate::basic_types::*;
use crate::basics::BasicStatement;

pub use operator::Operator;
use operations::parse_operations;
use tidying::tidy;
use procedures::parse_procedure_calls;
use parens::parse_parentheses;
use lists::parse_list;

// todo: breakup large functions into smaller bits
// todo: use more impls

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Raw(String),
    
    Operation {
        operator: Operator,
        args: Vec<Value>,
    },
    Operator(Operator),
    Group(Vec<Value>),
    ProcedureCall { name: String, args: Vec<Value> },

    I128(i128),
    Name(String),
    String(String),
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        let value = tokenize(value);
    
        parse_tokenized_value(value)
    }
}

/// Does every step of actually making a syntax tree except intial tokenization, which is handled by `From<&str> for Value`.
/// 
/// Exposed in order to facilitate making lists, because they need to recurse after the stuffs already been made into tokens.
fn parse_tokenized_value(value: Vec<Value>) -> Value {
    if let Some(value) = parse_list(&value) { return value; };
    
    let value = parse_parentheses(value);
    let value = parse_procedure_calls(value);
    let value      = parse_operations(value);

    // remove unnecesary Vecs and put in the final types
    let value      = tidy(value);

    value
}

/// Main function of the `Values` module.
/// 
/// Parses all of the `Value`s in a `Vec` of `BasicStatement`s from `Value::Raw`s
/// into full abstract syntax trees.
/// 
/// Mostly just deconstructs `BasicStatement`s, parses the `Value::Raw`s in them, and repackages them back into `BasicStatement`s
pub fn parse_values(statements: Vec<BasicStatement>) -> Vec<BasicStatement> {
    statements
        .into_iter()
        .map(|statement| {
            match statement {
                BasicStatement::Assignment { name, value } => {
                    BasicStatement::Assignment { 
                        name: name.parse_from_raw(), 
                        value: value.parse_from_raw() 
                    }
                },

                BasicStatement::Procedure { name, args, code, value } => {
                    BasicStatement::Procedure { 
                        name: name.parse_from_raw(), 
                        args: args.parse_from_raw(), 
                        code: parse_values(code), 
                        value: value.parse_from_raw() 
                    }
                },

                BasicStatement::Return { value } => {
                    BasicStatement::Return { 
                        value: value.parse_from_raw() 
                    }
                }

                BasicStatement::Control { kind, values, block } => {
                    BasicStatement::Control { 
                        kind, 
                        values: values
                            .into_iter()
                            .map(|v| Value::parse_from_raw(&v))
                            .collect::<Vec<_>>(), 
                        block: parse_values(block) 
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}


impl Value {
    /// Parses a `Value::Raw` into a full AST value
    /// `Value::Raw` is only used in intermediate steps after `BasicStatement`ing and before `Value`ing.
    /// 
    /// ### Panics
    /// Panics if the value is not a `Value::Raw`
    pub fn parse_from_raw(&self) -> Value {
        let Value::Raw(string) = self
            else { panic!("parse_raw_value must get a Value::Raw")};
    
        Value::from(&**string)
    }

    /// Always returns a `Vec::String`.
    /// 
    /// Applies a closure to every subelement, starting with the most deeply nested.
    /// 
    /// Will call the closure on parents as well as children.
    
    // todo: less copying here
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

    /// Like `apply_to_all`, but only returns the value if it's a `Value::String`.
    /// 
    /// Will therefore only be called on leaf nodes in the AST.
    // note: clones too much
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

/// Parses an `&str` into initial values
/// Only outputs a one-dimensional vec of tokens
/// This output is used by other functions to parse it into a full AST
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
