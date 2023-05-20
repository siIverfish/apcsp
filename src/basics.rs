use crate::lines::LineToken;
use crate::values::Value;

#[derive(Debug, Eq, PartialEq)]
pub enum ControlKind {
    If,
    ElseIf,
    Else,
    RepeatTimes,
    RepeatUntil,
    ForEach,
}
use ControlKind::*;

impl ControlKind {
    // start, sep, end, result
    const FROM_STRING_MAP: [(&'static str, &'static str, &'static str, ControlKind); 6] = [
        ("IF",           "___NO_SEPARATOR", "",      If),
        ("ELSE IF",      "___NO_SEPARATOR", "",      ElseIf),
        ("ELSE",         "___NO_SEPARATOR", "",      Else),
        ("REPEAT",       "___NO_SEPARATOR", "TIMES", RepeatTimes),
        ("REPEAT UNTIL", "___NO_SEPARATOR", "",      RepeatUntil),
        ("FOR EACH",     "IN",              "",      ForEach),
    ];
}

impl TryFrom<&str> for ControlKind {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::FROM_STRING_MAP
            .into_iter()
            .find(|map| 
                value.starts_with(map.0) &&
                value.ends_with(map.2)
            )
            .map(|m| m.3)
            .map_or(Err(()), Ok)
    }
}

impl ControlKind {
    fn mapping(&self) -> (&'static str, &'static str, &'static str, ControlKind) {
        Self::FROM_STRING_MAP
            .into_iter()
            .find(|map| &map.3 == self)
            .unwrap()
    }
    
    fn values_from(&self, line: &str) -> Option<Vec<Value>> {
        let mapping = self.mapping();

        line
            .strip_prefix(mapping.0)
            .and_then(|line| line.strip_suffix(mapping.2))
            .map(     |line| line.split(mapping.1))
            .map(|strs| 
                strs
                    .map(String::from)
                    .map(Value::Raw)
                    .collect::<Vec<_>>()
            )
    }
}

#[derive(Debug)]
pub enum BasicStatement {
    Assignment { name: Value, value: Value },
    Procedure  { name: Value, args: Value, code: Vec<BasicStatement>, value: Value },
    Control    { kind: ControlKind, values: Vec<Value>, block: Vec<BasicStatement> },
    Return     { value: Value },
}
use BasicStatement::*;

pub fn lines_to_statements(line_token: LineToken) -> Vec<BasicStatement> {
    let LineToken::Block(line_tokens) = line_token 
        else { panic!("lines_to_statements must take a LineToken::Block") };

    let mut line_tokens = line_tokens.into_iter();
    parse_line_tokens(&mut line_tokens)
}

// this function is kind of large
fn parse_line_tokens(line_tokens: &mut dyn Iterator<Item = LineToken>) -> Vec<BasicStatement> {
    let mut basic_statements: Vec<BasicStatement> = Vec::new();

    // this should never run into a Block 
    // because it'll consume those after procedure lines
    while let Some(LineToken::Normal(line)) = line_tokens.next() {
        if line.starts_with("PROCEDURE ") {
            basic_statements.push(parse_procedure(line, line_tokens));

        } else if let Some(value) = line.strip_prefix("RETURN") {
            basic_statements.push( Return { value: Value::Raw(value.trim().to_string()) } )

        } else if let Some((name, value)) = line.split_once("<-") {
            // this is an assigment

            basic_statements.push(Assignment{ 
                name:  Value::Raw(name .trim().to_string()), 
                value: Value::Raw(value.trim().to_string()),
            });
        } else if let Ok(kind) = ControlKind::try_from(&*line) {
            let values = kind.values_from(&line).unwrap();
            let Some(LineToken::Block(block)) = line_tokens.next()
                else { panic!("control statement missing block! (the {{ character should be on a newline afterwards)") };
            basic_statements.push(
                Control { kind, values, block: parse_line_tokens(&mut block.into_iter()) }
            )
        } else {
            // this is a value that will be turned into an '_' assignment for simplicity
            let assignment = Assignment { name: Value::Raw("_".into()), value: Value::Raw(line) };
            basic_statements.push(assignment);
        }
    }

    if line_tokens.next().is_some() {
        panic!("unexpected block!"); // i should make this message better probably
    }

    basic_statements
}

fn parse_procedure(line: String, line_tokens: &mut dyn Iterator<Item = LineToken>) -> BasicStatement {
    // procedure
    let Some((name, args)) = line
        .strip_prefix("PROCEDURE ")
        .map(str::trim)
        .and_then(|s| s.strip_suffix(')'))
        .and_then(|s| s.split_once('('))
        else { panic!("malformed procedure"); };
    
    let (name, args) = (Value::Raw(name.trim().to_string()), Value::Raw(args.trim().to_string()));
    
    let Some(LineToken::Block(code)) 
        = line_tokens.next()
        else { panic!("procedure missing code block") };
    
    let code = parse_line_tokens(&mut code.into_iter());
    let value = 
        if let Some( Return {value} ) = code.last() {
            value.clone()
        } else { 
            // this still has to be a string because that's
            // what the value parser expects everything to be initially
            Value::Raw("0".to_string()) 
        };
    
    Procedure { name, args, code, value }
}