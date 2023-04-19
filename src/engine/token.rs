#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Eof,
    Illegal,
    Comma,
    Dot,
    Lparen,
    Rparen,
    Integer,
    Identifire,
    StringObject,
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
    Greater,
    GreaterEq,
    Less,
    LessEq,
    True,
    False,
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

    pub fn literal(&self) -> String {
        self.literal.clone()
    }
}
