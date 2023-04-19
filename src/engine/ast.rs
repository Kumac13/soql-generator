use crate::engine::token::{Token, TokenKind};
use core::fmt::Debug;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + Debug {
    fn statement_node(&self);
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Table {
    pub token: Token,
    pub table_name: String,
}

impl Node for Table {
    fn token_literal(&self) -> String {
        self.table_name.clone()
    }
}

impl Statement for Table {
    fn statement_node(&self) {}
}

// <program> := <table> <statement>*
// <table> := <identifier>
// <statement> := <select_statement> | <where_statement> | <orderby_statement> | <groupby_statement> | <limit_statement> | <open_statement>
// <select_statement> := "." "select" "(" <field> ("," <field>)* ")"
// <where_statement> := "." "where" "(" <where_expression> ")"
// <orderby_statement> := "." "orderby" "(" <orderby_option> ("," <orderby_option>)* ")"
// <orderby_option> := <field> | <field> <order>
// <order> := "ASC" | "DESC"
// <groupby_statement> := "." "groupby" "(" <field> ("," <field>)* ")"
// <open_statement> := "." "open" "(" ")"
// <limit_statement> := "." "limit" "(" <integer> ")"
// <field> := <identifier> | <identifier> "." <identifier>
// <where_expression> := <expression> | <expression> "AND" <expression> | <expression> "OR" <expression>
// <expression> := <condition> | <condition> "AND" <condition> | <condition> "OR" <condition>
// <condition> := <field> | <field> <equal> <string> | <field> <like> <string> | <field> <equal> <integer> | <field> <like> <string> | <field> <operator> <boolean>
// <boolean> := "true" | "false"
// <equal> := "=" | "!="
// <like> := "LIKE"
// <greater> := ">" | ">=" | "<" | "<="
// <integer> := <digit>+
// <string> := <character>+
// <identifier> := <alpha> <alphanumeric>*
// <alpha> := a|b|...|z|A|B|...|Z|_
// <digit> := 0|1|...|9
// <alphanumeric> := <alpha>|<digit>
// <operator> := <greater>
