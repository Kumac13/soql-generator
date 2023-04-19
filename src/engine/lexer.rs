use crate::engine::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(input: &str) -> Vec<Token> {
    enum State {
        Normal,
        Orderby,
    }
    let mut tokens = Vec::new();
    let mut input = input.chars().peekable();

    while let Some(c) = input.next() {
        if c.is_whitespace() {
            continue;
        }

        match c {
            '=' => tokens.push(Token::new(TokenKind::Eq, String::from("="))),
            ',' => tokens.push(Token::new(TokenKind::Comma, String::from(","))),
            '.' => tokens.push(Token::new(TokenKind::Dot, String::from("."))),
            '(' => tokens.push(Token::new(TokenKind::Lparen, String::from("("))),
            ')' => tokens.push(Token::new(TokenKind::Rparen, String::from(")"))),
            '!' => {
                if let Some(c) = input.peek() {
                    if *c == '=' {
                        tokens.push(Token::new(TokenKind::NotEq, String::from("!=")));
                        input.next();
                    } else {
                        // TODO: error
                        tokens.push(Token::new(TokenKind::Bang, String::from("!")));
                    }
                } else {
                    // TDDO: error
                    tokens.push(Token::new(TokenKind::Bang, String::from("!")));
                }
            }
            '\'' => {
                let mut string_obj = consume_string_object(&mut input);
                tokens.push(Token::new(TokenKind::StringObject, string_obj));
            }
            _ => {
                if c.is_ascii_digit() {
                    tokens.push(Token::new(
                        TokenKind::Integer,
                        consume_integer(&mut input, c),
                    ));
                } else if is_literal(c) {
                    let literal = consume_literal(&mut input, c);
                    tokens.push(search_keywords(&literal));
                } else {
                    tokens.push(Token::new(TokenKind::Illegal, String::from(c)));
                }
            }
        }
    }
    tokens
}

fn consume_integer(input: &mut Peekable<Chars>, current_c: char) -> String {
    let mut num = String::from(current_c);
    while let Some(c) = input.peek() {
        if c.is_ascii_digit() {
            num.push(*c);
            input.next();
        } else {
            break;
        }
    }
    num
}

fn consume_literal(input: &mut Peekable<Chars>, current_c: char) -> String {
    let mut literal = String::from(current_c);
    while let Some(c) = input.peek() {
        if is_literal(*c) {
            literal.push(*c);
            input.next();
        } else {
            break;
        }
    }
    literal
}

fn consume_string_object(input: &mut Peekable<Chars>) -> String {
    let mut string_obj = String::new();
    while let Some(c) = input.next() {
        if c == '\'' {
            break;
        }
        string_obj.push(c);
        continue;
    }
    string_obj
}

fn is_literal(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn search_keywords(literal: &str) -> Token {
    match literal {
        "select" => Token::new(TokenKind::Select, String::from(literal)),
        "where" => Token::new(TokenKind::Where, String::from(literal)),
        "orderby" => Token::new(TokenKind::Orderby, String::from(literal)),
        "groupby" => Token::new(TokenKind::Groupby, String::from(literal)),
        "limit" => Token::new(TokenKind::Limit, String::from(literal)),
        "open" => Token::new(TokenKind::Open, String::from(literal)),
        "and" | "AND" => Token::new(TokenKind::And, String::from(literal)),
        "or" | "OR" => Token::new(TokenKind::Or, String::from(literal)),
        "like" | "LIKE" => Token::new(TokenKind::Like, String::from(literal)),
        "asc" | "ASC" => Token::new(TokenKind::Asc, String::from(literal)),
        "desc" | "DESC" => Token::new(TokenKind::Desc, String::from(literal)),
        _ => Token::new(TokenKind::Identifire, String::from(literal)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_only_table_name() {
        let input = "Account";

        let tokens = tokenize(input);
        assert_eq!(
            tokens[0],
            Token::new(TokenKind::Identifire, String::from("Account"))
        );
    }

    #[test]
    fn test_tokenize() {
        let input = "Account.select(Id, Name).where(Id = 1 AND ( Name LIKE '%hoge%' OR Name LIKE '%fuga%').orderby(Id, Name DESC).limit(10)";
        let expected = vec![
            Token::new(TokenKind::Identifire, String::from("Account")),
            Token::new(TokenKind::Dot, String::from(".")),
            Token::new(TokenKind::Select, String::from("select")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Identifire, String::from("Id")),
            Token::new(TokenKind::Comma, String::from(",")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::Dot, String::from(".")),
            Token::new(TokenKind::Where, String::from("where")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Identifire, String::from("Id")),
            Token::new(TokenKind::Eq, String::from("=")),
            Token::new(TokenKind::Integer, String::from("1")),
            Token::new(TokenKind::And, String::from("AND")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::Like, String::from("LIKE")),
            Token::new(TokenKind::StringObject, String::from("%hoge%")),
            Token::new(TokenKind::Or, String::from("OR")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::Like, String::from("LIKE")),
            Token::new(TokenKind::StringObject, String::from("%fuga%")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::Dot, String::from(".")),
            Token::new(TokenKind::Orderby, String::from("orderby")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Identifire, String::from("Id")),
            Token::new(TokenKind::Comma, String::from(",")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::Desc, String::from("DESC")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::Dot, String::from(".")),
            Token::new(TokenKind::Limit, String::from("limit")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Integer, String::from("10")),
            Token::new(TokenKind::Rparen, String::from(")")),
        ];

        let tokens = tokenize(input);
        assert_eq!(tokens, expected);
    }
}
