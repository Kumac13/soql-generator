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
                    "Unexpected token: expected {}. got \'{}\'",
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
        let iter = tokens.into_iter().peekable();
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
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        statements.push(self.parse_table()?);

        while let Some(token) = self.peek_token() {
            match token.kind {
                TokenKind::Eof => break,
                _ if token.is_query_method() => statements.push(self.parse_statement()?),
                _ => {
                    return Err(ParseError::InvalidMethod(
                        self.peek_token().unwrap().literal(),
                    ))
                }
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
                String::from("SObject Name"),
                self.current_token.literal(),
            ));
        }

        let table_name = self.current_token.literal();
        let token = self.current_token.clone();

        if !self.peek_token_is_query() {
            return Err(ParseError::UnexpectedToken(
                String::from("query method after SObject Name"),
                self.peek_token().unwrap().literal(),
            ));
        }
        Ok(Box::new(Table { token, table_name }))
    }

    // <statement> := <limit_statement> | <open_statement>
    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        match self.peek_token() {
            Some(token) => match token.kind {
                TokenKind::Select | TokenKind::Groupby => self.parse_select_groupby_statement(),
                TokenKind::Where => self.parse_where_statement(),
                TokenKind::Orderby => self.parse_orderby_statement(),
                TokenKind::Limit => self.parse_limit_statement(),
                TokenKind::Open => self.parse_open_statement(),
                _ => Err(ParseError::InvalidMethod(
                    self.peek_token().unwrap().literal(),
                )),
            },
            None => unreachable!(),
        }
    }

    // <select_statement> := 'select' '(' <field> (',' <field>)* ')'
    // <groupby_statement> := 'groupby' '(' <field> (',' <field>)* ')'
    fn parse_select_groupby_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.next_token().unwrap();

        self.expect_peek(TokenKind::Lparen)?;

        let fields = self.parse_fields()?;

        self.expect_peek(TokenKind::Rparen)?;

        let statement: Box<dyn Statement> = match token.kind {
            TokenKind::Select => Box::new(SelectStatement { token, fields }),
            TokenKind::Groupby => Box::new(GroupByStatement { token, fields }),
            _ => unreachable!(),
        };

        Ok(statement)
    }

    fn parse_where_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.next_token().unwrap();

        self.expect_peek(TokenKind::Lparen)?;

        let expression = self.parse_where_expressions()?;

        self.expect_peek(TokenKind::Rparen)?;

        Ok(Box::new(WhereStatement { token, expression }))
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

    fn parse_fields(&mut self) -> Result<Vec<FieldLiteral>, ParseError> {
        let mut fields = Vec::new();

        self.next_token();

        while !self.current_token_is(TokenKind::Rparen) {
            let field = self.parse_field()?;

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
    fn parse_field(&mut self) -> Result<FieldLiteral, ParseError> {
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
            let mut field = self.parse_field()?;

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

    // <where_expression> := <condition> | <grouped_condition>
    fn parse_where_expressions(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let mut left_exp = match self.peek_token() {
            Some(token) => match token.kind {
                TokenKind::Identifire => self.parse_condition()?,
                TokenKind::Lparen => self.parse_grouped_condition()?,
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        String::from("where clause"),
                        self.current_token.literal(),
                    ))
                }
            },
            None => {
                return Err(ParseError::UnexpectedToken(
                    String::from("where clause"),
                    self.current_token.literal(),
                ))
            }
        };

        while let Some(token) = self.peek_token() {
            match token.kind {
                TokenKind::And | TokenKind::Or => {
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                TokenKind::Rparen | TokenKind::Eof => {
                    break;
                }
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        String::from("where clause"),
                        self.current_token.literal(),
                    ))
                }
            }
        }

        Ok(left_exp)
    }

    // <infix_expression> := <where_expression> <operator> <where_expression>
    fn parse_infix_expression(
        &mut self,
        left: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, ParseError> {
        let infix_token = self.next_token().unwrap();
        let right = self.parse_where_expressions()?;

        Ok(Box::new(InfixExpression {
            token: infix_token.clone(),
            left,
            operator: infix_token.literal(),
            right,
        }))
    }

    // <condition> := <field> <operator> <value>
    fn parse_condition(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let token = self.next_token().unwrap();
        let field = self.parse_field()?;
        let operator = self.parse_operator_literal()?;
        let value = self.parse_value()?;

        Ok(Box::new(Condition {
            token,
            field,
            operator,
            value,
        }))
    }

    // <grouped_condition> := '(' <where_expression>')'
    fn parse_grouped_condition(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        self.next_token();

        let exp = self.parse_where_expressions()?;

        self.expect_peek(TokenKind::Rparen)?;

        Ok(exp)
    }

    fn parse_integer_literal(&mut self) -> Result<IntegerLiteral, ParseError> {
        let token = self.next_token().unwrap();
        let value = token.literal().parse::<i64>().unwrap();
        Ok(IntegerLiteral { token, value })
    }

    fn parse_operator_literal(&mut self) -> Result<OperatorLiteral, ParseError> {
        if let Some(token) = self.peek_token() {
            if token.is_operator() {
                self.next_token();
                let operator = OperatorLiteral {
                    token: self.current_token.clone(),
                    value: self.current_token.literal(),
                };
                Ok(operator)
            } else {
                return Err(ParseError::UnexpectedToken(
                    String::from("Operator(AND, OR, =, >, >=, <, <=, LIKE)"),
                    self.peek_token().unwrap().literal(),
                ));
            }
        } else {
            return Err(ParseError::UnexpectedToken(
                String::from("Operator(AND, OR, =, >, >=, <, <=, LIKE)"),
                self.peek_token().unwrap().literal(),
            ));
        }
    }

    fn parse_value(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        match self.peek_token() {
            Some(token) => match token.kind {
                TokenKind::Plus | TokenKind::Minus => self.parse_prefix_expression(),
                TokenKind::StringObject | TokenKind::Integer => Ok(Box::new(Value {
                    token: self.next_token().unwrap(),
                    value: self.current_token.literal(),
                })),
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        String::from(""),
                        self.peek_token().unwrap().literal(),
                    ))
                }
            },
            None => {
                return Err(ParseError::UnexpectedToken(
                    String::from(""),
                    self.peek_token().unwrap().literal(),
                ))
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let token = self.next_token().unwrap();
        let operator = token.literal();
        let right = self.parse_value()?;

        Ok(Box::new(PrefixExpression {
            token,
            operator,
            right,
        }))
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
        assert!(parser.parse().is_err());
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
            "Id, Name, Account.Name, Contract.LastName".to_string()
        );
    }

    #[test]
    fn test_parse_where() {
        let input =
            "Opportunity.where(Id = 123 AND (Name = 'test' OR Account.Name LIKE '%test%') AND Status = 'Closed')";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "where".to_string());

        assert_eq!(
            program.statements[1].string(),
            "(Id = 123 AND ((Name = 'test' OR Account.Name LIKE '%test%') AND Status = 'Closed'))"
                .to_string()
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
            "Id, Name, Account.Name".to_string()
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
            "Id, Name, Account.Name DESC".to_string()
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
        assert_eq!(program.statements[1].string(), "10".to_string());
    }

    #[test]
    fn test_parse_open() {
        let input = "Account.open()";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.statements[1].token_literal(), "open".to_string());
        assert_eq!(program.string(), "Account.open".to_string());
    }

}
