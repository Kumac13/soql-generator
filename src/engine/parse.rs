use crate::engine::ast::*;
use crate::engine::token::{Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

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

    pub fn parse(&mut self) -> Program {
        self.parse_program()
    }

    // <program> := <table> <statement>*
    fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        // first statement must be table name (identifier)
        if self.current_token_is(TokenKind::Identifire) {
            let table = self.parse_table();
            statements.push(table);
        } else {
            // TODO: parse error
            panic!("parse error");
        }

        Program { statements }
    }

    // <table> := <identifier>
    fn parse_table(&mut self) -> Box<dyn Statement> {
        let table_name = self.current_token.literal();
        self.next_token();

        Box::new(Table {
            token: self.current_token.clone(),
            table_name,
        })
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
            // TODO: parse error
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::lexer::tokenize;

    #[test]
    fn test_parse() {
        let input = "Produc2__c";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0].token_literal(),
            "Produc2__c".to_string()
        );
    }
}
