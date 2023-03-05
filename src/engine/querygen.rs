#[derive(Default, Debug)]
pub struct Query {
    pub select: Option<String>,
    pub from: String,
    pub where_clause: Option<String>,
    pub orderby: Option<String>,
    pub limit: Option<String>,
}

impl Query {
    pub fn generate(&self) -> String {
        let mut query = format!(
            "SELECT {} FROM {}",
            self.select.clone().unwrap_or_else(|| String::from("Id")),
            self.from
        );
        if let Some(where_clause) = &self.where_clause {
            query = format!("{} WHERE {} ", query, where_clause);
        }
        if let Some(orderby) = &self.orderby {
            query = format!("{} ORDER BY {}", query, orderby);
        }
        if let Some(limit) = &self.limit {
            query = format!("{} LIMIT {}", query, limit);
        }
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_query_with_defaults() {
        let query = Query::default();
        assert_eq!(query.generate(), "SELECT Id FROM ");
    }

    #[test]
    fn test_generate_query_with_select() {
        let mut query = Query::default();
        query.select = Some(String::from("Name, Age"));
        assert_eq!(query.generate(), "SELECT Name, Age FROM ");
    }

    #[test]
    fn test_generate_query_with_from() {
        let mut query = Query::default();
        query.from = String::from("Account");
        assert_eq!(query.generate(), "SELECT Id FROM Account");
    }

    #[test]
    fn test_generate_query_with_where() {
        let mut query = Query::default();
        query.where_clause = Some(String::from("Age > 18"));
        assert_eq!(query.generate(), "SELECT Id FROM  WHERE Age > 18 ");
    }

    #[test]
    fn test_generate_query_with_orderby() {
        let mut query = Query::default();
        query.orderby = Some(String::from("Name ASC"));
        assert_eq!(query.generate(), "SELECT Id FROM  ORDER BY Name ASC");
    }
}
