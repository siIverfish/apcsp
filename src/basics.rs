use crate::lines::LineToken;

#[derive(Debug)]
pub enum BasicStatement {
    Assignment { name: String, value: String },
    Procedure  { name: String, args: String, code: Vec<BasicStatement>, value: String },
    Return     { value: String },
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
            // procedure
            let Some((name, args)) = line
                .strip_prefix("PROCEDURE ")
                .unwrap()
                .trim()
                .strip_suffix(')')
                .expect("malformed procedure")
                .split_once('(')
                else { panic!("malformed procedure"); };
            
            let (name, args) = (name.trim().to_string(), args.trim().to_string());
            
            let Some(LineToken::Block(code)) 
                = line_tokens.next()
                else { panic!("procedure missing code block") };
            
            let mut code = parse_line_tokens(&mut code.into_iter());
            let value = 
                if let Some( Return {value} ) = code.pop() {
                    value
                } else { 
                    "0".to_string() 
                };
            
            let procedure = Procedure { name, args, code, value };

            basic_statements.push(procedure);

        } else if let Some(value) = line.strip_prefix("RETURN") {
            basic_statements.push( Return { value: value.trim().to_string() } )

        } else if let Some((name, value)) = line.split_once("<-") {
            // this is an assigment
            let (name, value) = (name.trim().to_string(), value.trim().to_string());

            basic_statements.push( Assignment { name, value });

        } else {
            // this is a value that will be turned into an '_' assignment for simplicity

            let assignment = Assignment { name: "_".into(), value: line };

            basic_statements.push(assignment);
        }
    }

    if line_tokens.next().is_some() {
        panic!("unexpected block!"); // i should make this message better probably
    }

    basic_statements
}