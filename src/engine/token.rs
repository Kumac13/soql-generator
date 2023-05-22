use std::fmt;

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
    Plus,
    Minus,
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
    Eq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    True,
    False,
    // Orderby Option
    Asc,
    Desc,
}

#[warn(unreachable_patterns)]
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Illegal => write!(f, "ILLEGAL"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Lparen => write!(f, "("),
            TokenKind::Rparen => write!(f, ")"),
            TokenKind::Integer => write!(f, "INTEGER"),
            TokenKind::Identifire => write!(f, "IDENTIFIRE"),
            TokenKind::StringObject => write!(f, "STRING"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Select => write!(f, "SELECT"),
            TokenKind::Where => write!(f, "WHERE"),
            TokenKind::Orderby => write!(f, "ORDERBY"),
            TokenKind::Groupby => write!(f, "GROUPBY"),
            TokenKind::Limit => write!(f, "LIMIT"),
            TokenKind::Open => write!(f, "OPEN"),
            TokenKind::And => write!(f, "AND"),
            TokenKind::Or => write!(f, "OR"),
            TokenKind::Like => write!(f, "LIKE"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEq => write!(f, ">="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEq => write!(f, "<="),
            TokenKind::True => write!(f, "TRUE"),
            TokenKind::False => write!(f, "FALSE"),
            TokenKind::Asc => write!(f, "ASC"),
            TokenKind::Desc => write!(f, "DESC"),
        }
    }
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

    pub fn is_query_method(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Select
                | TokenKind::Where
                | TokenKind::Orderby
                | TokenKind::Groupby
                | TokenKind::Limit
                | TokenKind::Open
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Or
                | TokenKind::And
                | TokenKind::Eq
                | TokenKind::NotEq
                | TokenKind::Greater
                | TokenKind::GreaterEq
                | TokenKind::Less
                | TokenKind::LessEq
                | TokenKind::Like
        )
    }

    pub fn is_dot(&self) -> bool {
        matches!(self.kind, TokenKind::Dot)
    }
}
