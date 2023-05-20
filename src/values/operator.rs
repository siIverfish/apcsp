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
    pub(crate) const STR_TO_OPERATOR: [(&'static str, Operator); 20] = [
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