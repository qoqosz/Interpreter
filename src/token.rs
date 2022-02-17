#[derive(Debug, PartialEq)]
pub enum Token {
    // default
    Illegal,
    EndOfFile,

    // number
    Integer(i64),

    // operations
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
}

impl Default for Token {
    fn default() -> Self {
        Token::Illegal
    }
}