
use std::{hash::Hash};

use crate::lexer::{Token};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Statement {
    LetStatement{
        token: Token,
        name: Expression,
        value: Expression
    },
    ReturnStatement {
        token: Token,
        return_value: Expression
    },
    ExpressionStatement {
        token: Token,
        expression: Expression
    },
    BlockStatment {
        token: Token,
        statements: Vec<Statement>
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Nope,
    Identifier {
        token: Token,
        value: String
    },
    IntegerLiteral {
        token: Token,
        value: i64
    },
    StringLiteral {
        token: Token,
        value: String
    },
    ArrayLiteral {
        token: Token,
        elements: Vec<Expression>
    },
    HashLiteral {
        token: Token,
        pairs: Vec<(Expression, Expression)>
    },
    IndexExpression {
        token: Token,
        left: Box<Expression>,
        index: Box<Expression>
    },
    PrefixExpression {
        token: Token,
        operator: String,
        right: Box<Expression>
    },
    InfixExpression {
        token: Token,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>
    },
    Boolean {
        token: Token,
        value: bool
    },
    IfExpression {
        token: Token,
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>
    },
    FunctionLiteral {
        token: Token,
        parameters: Vec<Expression>,
        body: Box<Statement>
    },
    CallExpression {
        token: Token,
        function: Box<Expression>,
        arguments: Vec<Expression>
    }
}