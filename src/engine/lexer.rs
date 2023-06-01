use crate::engine::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut input = input.chars().peekable();

    while let Some(c) = input.next() {
        if c.is_whitespace() {
            continue;
        }

        match c {
            '=' => tokens.push(Token::new(TokenKind::Eq, String::from("="))),
            // TODO: need to implement '+' and '-' for where condition
            '+' => tokens.push(Token::new(TokenKind::Plus, String::from("+"))),
            '-' => tokens.push(Token::new(TokenKind::Minus, String::from("-"))),
            '>' => {
                if let Some(c) = input.peek() {
                    if *c == '=' {
                        tokens.push(Token::new(TokenKind::GreaterEq, String::from(">=")));
                        input.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Greater, String::from(">")));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Greater, String::from(">")));
                }
            }
            '<' => {
                if let Some(c) = input.peek() {
                    if *c == '=' {
                        tokens.push(Token::new(TokenKind::LessEq, String::from("<=")));
                        input.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Less, String::from("<")));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Less, String::from("<")));
                }
            }
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
                        tokens.push(Token::new(TokenKind::Illegal, String::from("!")));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Illegal, String::from("!")));
                }
            }
            '\'' => {
                let string_obj = consume_string_object(&mut input);
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
                    let token = search_keywords(&literal);
                    if token.is_query_method() {
                        match tokens.pop() {
                            // the word before the query method must be a dot
                            Some(token) => {
                                if !token.is_dot() {
                                    eprintln!("Syntax error: the word before the query method must be a dot");
                                    std::process::exit(1);
                                }
                            }
                            _ => {
                                eprintln!(
                                    "Syntax error: the word before the query method must be a dot"
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                    tokens.push(token);
                } else {
                    tokens.push(Token::new(TokenKind::Illegal, String::from(c)));
                }
            }
        }
    }
    tokens.push(Token::new(TokenKind::Eof, String::from("")));
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
        if is_literal(*c) || c.is_ascii_digit() {
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
    for c in input.by_ref() {
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
        "true" | "TRUE" => Token::new(TokenKind::True, String::from(literal)),
        "false" | "FALSE" => Token::new(TokenKind::False, String::from(literal)),
        "null" | "NULL" => Token::new(TokenKind::Null, String::from(literal)),
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
        let input = "Opportunity.select(Id, Name, Account.Name).where(Id = 1 AND ( Name LIKE '%hoge%' OR Name LIKE '%fuga%' OR Name != NULL) AND CreatedDated >= '2022-11-10' AND IsPaid = TRUE OR Discount <= -1000).orderby(Id, Name DESC).limit(10).open()";
        let expected = vec![
            Token::new(TokenKind::Identifire, String::from("Opportunity")),
            Token::new(TokenKind::Select, String::from("select")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Identifire, String::from("Id")),
            Token::new(TokenKind::Comma, String::from(",")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::Comma, String::from(",")),
            Token::new(TokenKind::Identifire, String::from("Account")),
            Token::new(TokenKind::Dot, String::from(".")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::Rparen, String::from(")")),
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
            Token::new(TokenKind::Or, String::from("OR")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::NotEq, String::from("!=")),
            Token::new(TokenKind::Null, String::from("NULL")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::And, String::from("AND")),
            Token::new(TokenKind::Identifire, String::from("CreatedDated")),
            Token::new(TokenKind::GreaterEq, String::from(">=")),
            Token::new(TokenKind::StringObject, String::from("2022-11-10")),
            Token::new(TokenKind::And, String::from("AND")),
            Token::new(TokenKind::Identifire, String::from("IsPaid")),
            Token::new(TokenKind::Eq, String::from("=")),
            Token::new(TokenKind::True, String::from("TRUE")),
            Token::new(TokenKind::Or, String::from("OR")),
            Token::new(TokenKind::Identifire, String::from("Discount")),
            Token::new(TokenKind::LessEq, String::from("<=")),
            Token::new(TokenKind::Minus, String::from("-")),
            Token::new(TokenKind::Integer, String::from("1000")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::Orderby, String::from("orderby")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Identifire, String::from("Id")),
            Token::new(TokenKind::Comma, String::from(",")),
            Token::new(TokenKind::Identifire, String::from("Name")),
            Token::new(TokenKind::Desc, String::from("DESC")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::Limit, String::from("limit")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Integer, String::from("10")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::Open, String::from("open")),
            Token::new(TokenKind::Lparen, String::from("(")),
            Token::new(TokenKind::Rparen, String::from(")")),
            Token::new(TokenKind::Eof, String::from("")),
        ];

        let tokens = tokenize(input);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_consume_ineger() {
        let mut input = "1234567890".chars().peekable();
        input.next();
        let num = consume_integer(&mut input, '1');
        assert_eq!(num, "1234567890");
    }

    #[test]
    fn test_consume_literal() {
        let mut input = "Account".chars().peekable();
        input.next();
        let literal = consume_literal(&mut input, 'A');
        assert_eq!(literal, "Account");

        // case: literal with underscore and integer in the middle
        let mut input = "Product2__c".chars().peekable();
        input.next();
        let literal = consume_literal(&mut input, 'P');
        assert_eq!(literal, "Product2__c");
    }

    #[test]
    fn test_consume_string_object() {
        let mut input = "'%Test'".chars().peekable();
        input.next();
        let string_obj = consume_string_object(&mut input);
        assert_eq!(string_obj, "%Test");
    }
}
