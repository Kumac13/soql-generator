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
        Parser {
            tokens: iter,
            current_token: Token::new(TokenKind::Illegal, String::from("")),
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

        // parse table
        statements.push(self.parse_table()?);

        // parse statements
        while let Some(token) = self.peek_token() {
            if token.is_query_method() {
                statements.push(self.parse_statement()?);
            } else if token.kind == TokenKind::Eof {
                break;
            } else {
                return Err(ParseError::UnexpectedToken(
                    String::from("query method"),
                    token.literal(),
                ));
            }
        }

        Ok(Program { statements })
    }

    // <table> := <identifier>
    fn parse_table(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        self.next_token();

        // first statement must be table name (identifier)
        if !self.current_token_is(TokenKind::Identifire) {
            return Err(ParseError::UnexpectedToken(
                String::from("Table"),
                self.current_token.literal(),
            ));
        }

        let token = self.current_token.clone();
        let table_name = self.current_token.literal();

        if !self.peek_token_is(TokenKind::Eof) && !self.peek_token_is_query() {
            return Err(ParseError::UnexpectedToken(
                String::from("EOF or query method after SObject Name"),
                self.current_token.literal(),
            ));
        }
        Ok(Box::new(Table { token, table_name }))
    }

    // <statement> := <limit_statement> | <open_statement>
    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        match self.peek_token() {
            Some(token) => match token.kind {
                TokenKind::Select | TokenKind::Groupby => self.parse_select_groupby_statement(),
                TokenKind::Orderby => self.parse_orderby_statement(),
                TokenKind::Limit => self.parse_limit_statement(),
                TokenKind::Open => self.parse_open_statement(),
                _ => Err(ParseError::InvalidMethod(
                    self.peek_token().unwrap().literal(),
                )),
            },
            None => Err(ParseError::InvalidMethod(String::from(""))),
        }
    }

    // <select_statement> := 'select' '(' <field> (',' <field>)* ')'
    // <groupby_statement> := 'groupby' '(' <field> (',' <field>)* ')'
    fn parse_select_groupby_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.next_token().unwrap();

        self.expect_peek(TokenKind::Lparen)?;

        let fields = self.parse_fileds()?;

        self.expect_peek(TokenKind::Rparen)?;

        let statement: Box<dyn Statement> = match token.kind {
            TokenKind::Select => Box::new(SelectStatement { token, fields }),
            TokenKind::Groupby => Box::new(GroupByStatement { token, fields }),
            _ => unreachable!(),
        };

        Ok(statement)
    }

    // <orderby_statement> := 'orderby' '(' <orderby_option> (',' <orderby_option>)* ')'
    fn parse_orderby_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.next_token().unwrap();

        self.expect_peek(TokenKind::Lparen)?;

        let options = self.parse_orderby_options()?;

        self.expect_peek(TokenKind::Rparen)?;

        Ok(Box::new(OrderByStatement { token, options }))
    }

    // <limit_statement> := 'limit' '(' <integer> ')'
    fn parse_limit_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.next_token().unwrap();

        self.expect_peek(TokenKind::Lparen)?;

        let limit = self.parse_integer_literal()?;

        self.expect_peek(TokenKind::Rparen)?;

        Ok(Box::new(LimitStatement { token, limit }))
    }

    // <open_statement> := 'open' '(' ')'
    fn parse_open_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.next_token().unwrap();

        self.expect_peek(TokenKind::Lparen)?;
        self.expect_peek(TokenKind::Rparen)?;

        Ok(Box::new(OpenStatement { token }))
    }

    fn parse_fileds(&mut self) -> Result<Vec<FieldLiteral>, ParseError> {
        let mut fields = Vec::new();

        self.next_token();

        while !self.peek_token_is(TokenKind::Rparen) {
            let field = self.parse_filed()?;

            if self.peek_token_is(TokenKind::Rparen) {
                fields.push(field);
                break;
            }

            self.expect_peek(TokenKind::Comma)?;

            self.next_token();

            fields.push(field);
        }

        Ok(fields)
    }

    // <field> := <identifier> | <identifire> <dot> <identifier>
    fn parse_filed(&mut self) -> Result<FieldLiteral, ParseError> {
        let token = self.current_token.clone();
        let mut name = self.current_token.literal();

        if self.peek_token_is(TokenKind::Dot) {
            self.next_token();

            self.expect_peek(TokenKind::Identifire)?;

            name = format!("{}.{}", name, self.current_token.literal());
        }

        Ok(FieldLiteral { token, name })
    }

    // <orderby_option> := <field> | <field> <asc_or_desc>
    fn parse_orderby_options(&mut self) -> Result<Vec<OrderByOptionLiteral>, ParseError> {
        let mut options = Vec::new();

        self.next_token();

        while !self.peek_token_is(TokenKind::Rparen) {
            let mut field = self.parse_filed()?;

            if self.peek_token_is(TokenKind::Asc) {
                self.next_token();
            } else if self.peek_token_is(TokenKind::Desc) {
                self.next_token();
                field.name = format!("{} {}", field.name, self.current_token.literal());
            }

            let option = OrderByOptionLiteral {
                token: field.token,
                name: field.name,
            };

            if self.peek_token_is(TokenKind::Rparen) {
                options.push(option);
                break;
            }

            self.expect_peek(TokenKind::Comma)?;

            self.next_token();

            options.push(option);
        }
        Ok(options)
    }

    fn parse_integer_literal(&mut self) -> Result<IntegerLiteral, ParseError> {
        let token = self.next_token().unwrap();
        let value = token.literal().parse::<i64>().unwrap();
        Ok(IntegerLiteral { token, value })
    }

    fn current_token_is(&mut self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }

    fn peek_token_is(&mut self, kind: TokenKind) -> bool {
        self.peek_token().map_or(false, |token| token.kind == kind)
    }

    fn peek_token_is_query(&mut self) -> bool {
        self.peek_token()
            .map_or(false, |token| token.is_query_method())
    }
    fn expect_peek(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.peek_token_is(kind.clone()) {
            self.next_token();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(
                kind.to_string(),
                self.peek_token().unwrap().literal(),
            ))
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
    fn test_parse_select() {
        let input = "Opportunity.select(Id, Name, Account.Name, Contract.LastName)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "select".to_string());
        assert_eq!(
            program.statements[1].string(),
            "select(Id, Name, Account.Name, Contract.LastName)".to_string()
        );
    }

    #[test]
    fn test_parse_groupby() {
        let input = "Opportunity.groupby(Id, Name, Account.Name)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "groupby".to_string());
        assert_eq!(
            program.statements[1].string(),
            "groupby(Id, Name, Account.Name)".to_string()
        );
    }

    #[test]
    fn test_parse_orderby() {
        let input = "Opportunity.orderby(Id, Name ASC, Account.Name DESC)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "orderby".to_string());
        assert_eq!(
            program.statements[1].string(),
            "orderby(Id, Name, Account.Name DESC)".to_string()
        );
    }

    #[test]
    fn test_parse_limit() {
        let input = "Account.limit(10)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "limit".to_string());
        assert_eq!(program.statements[1].string(), "limit(10)".to_string());
    }

    #[test]
    fn test_parse_open() {
        let input = "Account.open()";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "open".to_string());
        assert_eq!(program.statements[1].string(), "open".to_string());
    }
}
