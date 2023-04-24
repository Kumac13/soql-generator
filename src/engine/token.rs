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

impl TokenKind {
    pub fn to_string(&self) -> String {
        match &self {
            TokenKind::Eof => "EOF".to_string(),
            TokenKind::Illegal => "ILLEGAL".to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Dot => ".".to_string(),
            TokenKind::Lparen => "(".to_string(),
            TokenKind::Rparen => ")".to_string(),
            TokenKind::Integer => "INTEGER".to_string(),
            TokenKind::Identifire => "IDENTIFIRE".to_string(),
            TokenKind::StringObject => "STRING".to_string(),
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Select => "SELECT".to_string(),
            TokenKind::Where => "WHERE".to_string(),
            TokenKind::Orderby => "ORDERBY".to_string(),
            TokenKind::Groupby => "GROUPBY".to_string(),
            TokenKind::Limit => "LIMIT".to_string(),
            TokenKind::Open => "OPEN".to_string(),
            TokenKind::And => "AND".to_string(),
            TokenKind::Or => "OR".to_string(),
            TokenKind::Like => "LIKE".to_string(),
            TokenKind::Asc => "ASC".to_string(),
            TokenKind::Desc => "DESC".to_string(),
            TokenKind::Eq => "=".to_string(),
            TokenKind::NotEq => "!=".to_string(),
            TokenKind::Greater => ">".to_string(),
            TokenKind::GreaterEq => ">=".to_string(),
            TokenKind::Less => "<".to_string(),
            TokenKind::LessEq => "<=".to_string(),
            TokenKind::True => "TRUE".to_string(),
            TokenKind::False => "FALSE".to_string(),
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
