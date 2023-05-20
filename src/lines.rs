

#[derive(Debug)]
pub enum LineToken {
    Normal(String),
    Block(Vec<LineToken>),
}

pub fn code_to_lines(code: String) -> LineToken {
    let mut lines = code.lines();
    parse_lines(&mut lines)
}

fn is_incomplete(line: &str) -> bool {
    use crate::values::Operator;

    if line.chars().filter(|c| c == &'(').count() != 
        line.chars().filter(|c| c == &')').count()
    { true } 
    else if line.chars().filter(|c| c == &'[').count() != 
            line.chars().filter(|c| c == &']').count()
    { true } 
    else if Operator::STR_TO_OPERATOR.iter().map(|t| t.0).any(|operator| !"[]()".contains(operator) && line.ends_with(operator)) 
    { true } 
    else 
    { false }
}

fn parse_lines(lines: &mut dyn Iterator<Item = &str>) -> LineToken {
    let mut line_tokens: Vec<LineToken> = Vec::new();

    while let Some(line) = lines.next().map(str::trim) {
        let mut line_token = match line {
            ""           => continue,
            "}"          => return LineToken::Block(line_tokens),
            "{"          => parse_lines(lines),
            line   => LineToken::Normal(line.to_string()),
        };

        line_tokens.push(
            if let LineToken::Normal(ref mut string) = line_token {
                while is_incomplete(string) {
                    string.push_str(lines.next().map(str::trim).expect("incomplete value at end of file"));
                }
                line_token
            } else {
                line_token
            }
        )
    }

    LineToken::Block(line_tokens)
}
