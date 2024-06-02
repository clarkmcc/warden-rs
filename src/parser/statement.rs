use crate::parser::{Expression, Identifier};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Assignment {
        target: Expression,
        value: Expression,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Case {
        expr: Option<Expression>,
        clauses: Vec<(Expression, Statement)>,
        else_clause: Option<Box<Statement>>,
    },
    For {
        collection: Expression,
        key: Option<Identifier>,
        value: Identifier,
        body: Box<Statement>,
    },
    Break,
    Continue,
    Return(Option<Expression>),
}
