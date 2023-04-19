use crate::engine::ast::*;
use crate::engine::token::{Token, TokenKind};
use std::{
    error::Error,
    fmt::{self, Display},
    iter::Peekable,
    vec::IntoIter,
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String, String),
    InvalidMethod(String),
    Eof,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(message, token_literal) => {
                write!(
                    f,
                    "Unexpected token: expected {}. got {}",
                    message, token_literal
                )
            }
            ParseError::InvalidMethod(method) => {
                write!(f, "Invalid method: {}", method)
            }
            ParseError::Eof => write!(f, "Unexpected EOF"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Peekable<IntoIter<Token>>,
    pub current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter().peekable();
        let current_token = iter.next().unwrap();
        Parser {
            tokens: iter,
            current_token,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.current_token = self.tokens.next()?;
        Some(self.current_token.clone())
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    // <program> := <table> <statement>*
    fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        // first statement must be table name (identifier)
        if self.current_token_is(TokenKind::Identifire) {
            let table = self.parse_table()?;
            statements.push(table);
        } else {
            // TODO: parse error
            panic!("parse error");
        }

        // parse statements
        while let Some(token) = self.peek_token() {
            match token.kind {
                TokenKind::Dot => {
                    self.next_token();
                    let statement = self.parse_statement()?;
                    statements.push(statement);
                }
                TokenKind::Eof => {
                    break;
                }
                _ => return Err(ParseError::Eof),
            }
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        match self.peek_token() {
            Some(token) => match token.kind {
                TokenKind::Open => self.parse_open_statement(),
                _ => Err(ParseError::InvalidMethod(String::from("SELECT"))),
            },
            None => Err(ParseError::InvalidMethod(String::from(""))),
        }
    }

    fn parse_open_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.next_token().unwrap();

        if !self.expect_peek(TokenKind::Lparen) {
            return Err(ParseError::UnexpectedToken(
                String::from("\'(\'"),
                self.peek_token().unwrap().literal(),
            ));
        }

        if !self.expect_peek(TokenKind::Rparen) {
            return Err(ParseError::UnexpectedToken(
                String::from("\')\'"),
                self.peek_token().unwrap().literal(),
            ));
        }

        Ok(Box::new(OpenStatement { token }))
    }

    // <table> := <identifier>
    fn parse_table(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let table_name = self.current_token.literal();

        if !self.peek_token_is(TokenKind::Eof) && !self.peek_token_is(TokenKind::Dot) {
            return Err(ParseError::UnexpectedToken(
                String::from("EOF or \'.\'"),
                self.current_token.literal(),
            ));
        }
        Ok(Box::new(Table {
            token: self.current_token.clone(),
            table_name,
        }))
    }

    fn current_token_is(&mut self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }

    fn peek_token_is(&mut self, kind: TokenKind) -> bool {
        self.peek_token().map_or(false, |token| token.kind == kind)
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::lexer::tokenize;

    #[test]
    fn test_parse_talbe() {
        let input = "Produc2__c";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0].token_literal(),
            "Produc2__c".to_string()
        );
    }

    #[test]
    fn test_parse_open() {
        let input = "Account.open()";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "open".to_string());
    }
}
