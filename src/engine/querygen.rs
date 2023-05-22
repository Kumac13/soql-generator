use crate::engine::ast::*;
use crate::helper::DynError;

#[derive(Default, Debug)]
pub struct Query {
    pub select: Option<String>,
    pub from: String,
    pub where_clause: Option<String>,
    pub orderby: Option<String>,
    pub limit: Option<String>,
    pub open_browser: bool,
}

impl Query {
    pub fn generate(&self) -> String {
        let mut query = format!(
            "SELECT {} FROM {}",
            self.select.clone().unwrap_or_else(|| String::from("Id")),
            self.from
        );

        if let Some(where_clause) = &self.where_clause {
            query = format!("{} WHERE {}", query, where_clause);
        }

        if self.open_browser {
            query = format!("{} LIMIT 1", query);
            return query;
        }

        if let Some(orderby) = &self.orderby {
            query = format!("{} ORDER BY {}", query, orderby);
        }
        if let Some(limit) = &self.limit {
            query = format!("{} LIMIT {}", query, limit);
        }
        query
    }

    pub fn evaluate(&mut self, prgram: Program) -> Result<(), DynError> {
        for node in prgram.statements {
            self.evalute_statement(node)?;
        }
        Ok(())
    }

    fn evalute_statement(&mut self, node: Box<dyn Statement>) -> Result<(), DynError> {
        match node.node_type() {
            NodeType::Table => {
                self.from = node.string();
            }
            NodeType::SelectStatement => {
                self.select = Some(node.string());
            }
            NodeType::GroupByStatement => {
                self.select = Some(node.string());
            }
            NodeType::WhereStatement => {
                self.where_clause = Some(node.string());
            }
            NodeType::OrderByStatement => {
                self.orderby = Some(node.string());
            }
            NodeType::LimitStatement => {
                self.limit = Some(node.string());
            }
            NodeType::OpenStatement => {
                self.open_browser = true;
            }
            _ => {
                return Err("invalid node type".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::lexer::tokenize;
    use crate::engine::parse::Parser;

    #[test]
    fn test_evaluate_select() {
        let input = "Opportunity.select(Id, Name, Account.Name, Contract.LastName)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut query = Query::default();
        query.evaluate(program).unwrap();

        assert_eq!(
            query.select.unwrap(),
            "Id, Name, Account.Name, Contract.LastName".to_string()
        );
    }

    #[test]
    fn test_evaluate_groupby() {
        let input = "Opportunity.groupby(Id, Name, Account.Name, Contract.LastName)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut query = Query::default();
        query.evaluate(program).unwrap();

        assert_eq!(
            query.select.unwrap(),
            "Id, Name, Account.Name, Contract.LastName".to_string()
        );
    }

    #[test]
    fn test_evaluate_where() {
        let input = "Opportunity.where(Id = 123 AND (Name = 'test' OR Account.Name LIKE '%test%') AND Status = 'Closed')";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut query = Query::default();
        query.evaluate(program).unwrap();

        assert_eq!(
            query.where_clause.unwrap(),
            "(Id = 123 AND ((Name = 'test' OR Account.Name LIKE '%test%') AND Status = 'Closed'))"
                .to_string()
        );
    }

    #[test]
    fn test_evaluate_orderby() {
        let input = "Account.orderby(Id, Name ASC, Account.Name DESC)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut query = Query::default();
        query.evaluate(program).unwrap();

        assert_eq!(
            query.orderby.unwrap(),
            "Id, Name, Account.Name DESC".to_string()
        );
    }

    #[test]
    fn test_evaluate_limit() {
        let input = "Account.limit(10)";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut query = Query::default();
        query.evaluate(program).unwrap();

        assert_eq!(query.limit.unwrap(), "10");
    }

    #[test]
    fn test_evaluate_open() {
        let input = "Account.open()";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut query = Query::default();
        query.evaluate(program).unwrap();

        assert_eq!(query.from, "Account");
        assert_eq!(query.open_browser, true);
    }
}
