#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Eof,
    Illegal,
    Comma,
    Dot,
    Lparen,
    Rparen,
    Bang,
    Integer,
    Identifire,
    // Methods
    Select,
    Where,
    Orderby,
    Groupby,
    Limit,
    Open,
    // Method Operators
    And,
    Or,
    Like,
    Asc,
    Desc,
    Eq,
    NotEq,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}
