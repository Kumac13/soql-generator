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

    pub fn is_method(&self) -> bool {
        matches!(
            &self.kind,
            TokenKind::Select
                | TokenKind::Where
                | TokenKind::Orderby
                | TokenKind::Groupby
                | TokenKind::Limit
                | TokenKind::Open
        )
    }
}
