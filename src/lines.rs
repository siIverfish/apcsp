

#[derive(Debug)]
pub enum LineToken {
    Normal(String),
    Block(Vec<LineToken>),
}

pub fn code_to_lines(code: String) -> LineToken {
    let mut lines = code.lines();
    parse_lines(&mut lines)
}

fn parse_lines(lines: &mut dyn Iterator<Item = &str>) -> LineToken {
    let mut line_tokens: Vec<LineToken> = Vec::new();

    while let Some(line) = lines.next() {
        match line.trim() {
            ""           => (),
            "}"          => return LineToken::Block(line_tokens),
            "{"          => line_tokens.push( parse_lines(lines) ),
            line   => line_tokens.push( LineToken::Normal(line.to_string()) ),
        }
    }

    LineToken::Block(line_tokens)
}
