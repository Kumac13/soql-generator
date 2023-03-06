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
            _ => (),
        };

        if !used_methods.insert(method_condition) {
            return Err(ParseError::DuplicatedMethod(method.to_string()));
        }
    }

    Ok(query)
}

//pub fn parse(expr: &str) -> Result<QueryMethod, ParseError> {}

fn split_method_condition(part: &str) -> (&str, &str) {
    let parts: Vec<&str> = part.split('(').collect();
    let method = parts[0];
    let condition = parts[1].trim_end_matches(')');

    (method, condition)
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
            "Opportunity.select(name).where(amount > 1000).orderby(created_date).limit(10)";
        let query = parse(query_str).unwrap();
        assert_eq!(query.from, "Opportunity");
        assert_eq!(query.select.unwrap(), "name");
        assert_eq!(query.where_clause.unwrap(), "amount > 1000");
        assert_eq!(query.orderby.unwrap(), "created_date");
        assert_eq!(query.limit.unwrap(), "10");
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
