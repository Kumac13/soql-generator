use crate::engine::querygen::Query;
use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum ParseError {
    InvalidSObject,
    InvalidOption,
    InvalidLimitOption(String),
    Empty,
    DuplicatedMethod(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidSObject => {
                write!(f, "ParseError: invalid sobject")
            }
            ParseError::InvalidOption => {
                write!(f, "ParseError: invalid option")
            }
            ParseError::InvalidLimitOption(s) => {
                write!(f, "ParseError: limit option need to integer: {}", s)
            }
            ParseError::Empty => write!(f, "ParseError: empty option"),
            ParseError::DuplicatedMethod(s) => {
                write!(f, "ParseError: method `{}` is duplicated", s)
            }
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MethodCondition {
    Select,
    Where,
    Order,
    Limit,
    Open,
}

pub fn parse(expr: &str) -> Result<Query, ParseError> {
    let mut query = Query::default();
    let mut used_methods = HashSet::new();

    let parts: Vec<&str> = expr.split('.').collect();
    query.from = if parts[0].contains('(') {
        return Err(ParseError::InvalidSObject);
    } else {
        parts[0].to_owned()
    };

    for part in parts[1..].iter() {
        let (method, condition) = split_method_condition(part.trim());

        let method_condition = match method {
            "select" => MethodCondition::Select,
            "where" => MethodCondition::Where,
            "orderby" => MethodCondition::Order,
            "limit" => MethodCondition::Limit,
            "open" => MethodCondition::Open,
            _ => return Err(ParseError::InvalidOption),
        };

        match method_condition {
            MethodCondition::Select => {
                query.select = Some(parse_select(condition));
            }
            MethodCondition::Where => {
                query.where_clause = Some(parse_where(condition).to_owned());
            }
            MethodCondition::Order => {
                query.orderby = Some(condition.to_owned());
            }
            MethodCondition::Limit => {
                query.limit = Some(parse_limit(condition)?);
            }
            MethodCondition::Open => {
                query.open_browser = true;
            }
            _ => (),
        };

        if !used_methods.insert(method_condition) {
            return Err(ParseError::DuplicatedMethod(method.to_string()));
        }
    }

    Ok(query)
}

//pub fn parse(expr: &str) -> Result<QueryMethod, ParseError> {}

/*
fn split_method_condition(part: &str) -> (&str, &str) {
    let parts: Vec<&str> = part.split('(').collect();

    println!("{:?}", parts);

    let method = parts[0];
    let condition = parts[1].trim_end_matches(')');

    (method, condition)
}
*/
fn split_method_condition(part: &str) -> (&str, &str) {
    let mut method = "";
    let mut condition = "";
    let mut depth = 0;

    for (i, c) in part.char_indices() {
        if c == '(' {
            depth += 1;
            if depth == 1 {
                method = &part[..i];
            }
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                condition = &part[(method.len() + 1)..i];
                break;
            }
        }
    }

    (method.trim(), condition.trim())
}

fn parse_select(condition: &str) -> String {
    condition.to_string()
}

fn parse_where(condition: &str) -> String {
    condition.to_string()
}

fn parse_limit(condition: &str) -> Result<String, ParseError> {
    if condition.is_empty() {
        return Err(ParseError::Empty);
    }
    match condition.parse::<i32>() {
        Ok(limit) => Ok(limit.to_string()),
        Err(_) => Err(ParseError::InvalidLimitOption(condition.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let query_str =
            "Opportunity.select(name).where(name like 'test').orderby(created_date).limit(10).open()";
        let query = parse(query_str).unwrap();
        assert_eq!(query.from, "Opportunity");
        assert_eq!(query.select.unwrap(), "name");
        assert_eq!(query.where_clause.unwrap(), "name like 'test'");
        assert_eq!(query.orderby.unwrap(), "created_date");
        assert_eq!(query.limit.unwrap(), "10");
        assert_eq!(query.open_browser, true);
    }

    #[test]
    fn test_parse_where_clause() {
        // Case1: simple pattern
        let query_str = "Account.where(name like '%test%')";
        let result = parse(query_str).unwrap();

        assert_eq!(result.where_clause.unwrap(), "name like '%test%'");

        // Casel2: complecated pattern
        let query_str = "Account.where(name = 'test' and (id = 10 or id = 20))";
        let result = parse(query_str).unwrap();

        assert_eq!(
            result.where_clause.unwrap(),
            "name = 'test' and (id = 10 or id = 20)"
        );
    }

    #[test]
    fn test_parse_invalid_sobject() {
        let query_str = "select(Id, Name)";
        let result = parse(query_str);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_limit() {
        let query_str = "Opportunity.limit(hoge)";
        let result = parse(query_str);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_duplicated_method() {
        let query_str = "Opportunity.where(id = 10).where(id = 20)";
        let result = parse(query_str);

        assert!(result.is_err());
    }
}
