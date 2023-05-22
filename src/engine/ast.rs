use crate::engine::token::{Token, TokenKind};
use core::fmt::Debug;
use std::any::Any;

pub enum NodeType {
    Program,
    Table,
    SelectStatement,
    WhereStatement,
    GroupByStatement,
    OrderByStatement,
    LimitStatement,
    OpenStatement,
    CloseStatement,
    FieldLiteral,
    OrderByOptionLiteral,
    IntegerLiteral,
    StringLiteral,
    BooleanLiteral,
    NullLiteral,
    Value,
    PrefixExpression,
    InfixExpression,
    Condition,
    OperatorLiteral,
}

pub trait Node: Any {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn node_type(&self) -> NodeType;
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
        if !self.statements.is_empty() {
            let literals = self
                .statements
                .iter()
                .map(|s| s.token_literal())
                .collect::<Vec<String>>();
            literals.join(".")
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        if !self.statements.is_empty() {
            let literals = self
                .statements
                .iter()
                .map(|s| s.string())
                .collect::<Vec<String>>();
            literals.join(".")
        } else {
            "".to_string()
        }
    }

    fn node_type(&self) -> NodeType {
        NodeType::Program
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

    fn string(&self) -> String {
        self.table_name.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Table
    }
}

impl Statement for Table {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct SelectStatement {
    pub token: Token,
    pub fields: Vec<FieldLiteral>,
}

impl Node for SelectStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut s = self.token_literal();
        let params: Vec<String> = self.fields.iter().map(|f| f.string()).collect();
        params.join(", ")
    }

    fn node_type(&self) -> NodeType {
        NodeType::SelectStatement
    }
}

impl Statement for SelectStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct WhereStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for WhereStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut s = self.token_literal();
        s += "(";
        s += &self.expression.string();
        s += ")";
        s
    }

    fn node_type(&self) -> NodeType {
        NodeType::WhereStatement
    }
}

impl Statement for WhereStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct GroupByStatement {
    pub token: Token,
    pub fields: Vec<FieldLiteral>,
}

impl Node for GroupByStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut s = self.token_literal();
        let params: Vec<String> = self.fields.iter().map(|f| f.string()).collect();
        s += "(";
        s += &params.join(", ");
        s += ")";
        s
    }

    fn node_type(&self) -> NodeType {
        NodeType::GroupByStatement
    }
}
impl Statement for GroupByStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct OrderByStatement {
    pub token: Token,
    pub options: Vec<OrderByOptionLiteral>,
}

impl Node for OrderByStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut s = self.token_literal();
        let params: Vec<String> = self.options.iter().map(|f| f.string()).collect();
        params.join(", ")
    }

    fn node_type(&self) -> NodeType {
        NodeType::OrderByStatement
    }
}

impl Statement for OrderByStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct LimitStatement {
    pub token: Token,
    pub limit: IntegerLiteral,
}

impl Node for LimitStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.limit.string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::LimitStatement
    }
}

impl Statement for LimitStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct OpenStatement {
    pub token: Token,
}

impl Node for OpenStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.token_literal()
    }

    fn node_type(&self) -> NodeType {
        NodeType::OpenStatement
    }
}

impl Statement for OpenStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::IntegerLiteral
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct FieldLiteral {
    pub token: Token,
    pub name: String,
}

impl Node for FieldLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.name.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::FieldLiteral
    }
}

impl Expression for FieldLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct OrderByOptionLiteral {
    pub token: Token,
    pub name: String,
}

impl Node for OrderByOptionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.name.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::OrderByOptionLiteral
    }
}

impl Expression for OrderByOptionLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.value.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::StringLiteral
    }
}

impl Expression for StringLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::BooleanLiteral
    }
}

impl Expression for BooleanLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct OperatorLiteral {
    pub token: Token,
    pub value: String,
}

impl Node for OperatorLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.value.clone()
    }

    fn node_type(&self) -> NodeType {
        NodeType::OperatorLiteral
    }
}

impl Expression for OperatorLiteral {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct Value {
    pub token: Token,
    pub value: String,
}

impl Node for Value {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        match self.token.kind {
            TokenKind::Identifire | TokenKind::StringObject => format!("\'{}\'", self.value),
            _ => self.value.clone(),
        }
    }

    fn node_type(&self) -> NodeType {
        NodeType::Value
    }
}

impl Expression for Value {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut s = "(".to_string();
        s += &self.operator;
        s += &self.right.string();
        s += ")";
        s
    }

    fn node_type(&self) -> NodeType {
        NodeType::PrefixExpression
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut s = "(".to_string() + &self.left.string();
        s += " ";
        s += &self.operator;
        s += " ";
        s += &self.right.string();
        s += ")";
        s
    }

    fn node_type(&self) -> NodeType {
        NodeType::InfixExpression
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct Condition {
    pub token: Token,
    pub field: FieldLiteral,
    pub operator: OperatorLiteral,
    pub value: Box<dyn Expression>,
}

impl Node for Condition {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut s = self.field.string();
        s += " ";
        s += &self.operator.string();
        s += " ";
        s += &self.value.string();
        s
    }

    fn node_type(&self) -> NodeType {
        NodeType::Condition
    }
}

impl Expression for Condition {
    fn expression_node(&self) {}
}
