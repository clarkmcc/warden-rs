use crate::parser::{
    BinaryOperator, Identifier, Literal, Parsable, ParsableError, QuantifierType, UnaryOperator,
};
use chumsky::pratt::{infix, Associativity};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    UnaryExpr {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    BinaryExpr {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    Call {
        func: Identifier,
        args: Vec<Expression>,
    },
    Index {
        collection: Box<Expression>,
        index: Box<Expression>,
    },
    Slice {
        collection: Box<Expression>,
        start: Option<Box<Expression>>,
        end: Option<Box<Expression>>,
    },
    Select {
        object: Box<Expression>,
        field: Identifier,
    },
    List(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
    Rule {
        when: Option<Box<Expression>>,
        body: Box<Expression>,
    },
    Quantifier {
        quant: QuantifierType,
        collection: Box<Expression>,
        key: Option<Identifier>,
        value: Identifier,
        body: Box<Expression>,
    },
}

impl Expression {
    pub fn literal<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        Literal::parser().map(Expression::Literal)
    }

    pub fn identifier<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        Identifier::parser().map(Expression::Identifier)
    }

    pub fn function<'src>(
        args: impl Parser<'src, &'src str, Self, ParsableError<'src>>,
    ) -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        Identifier::parser()
            .then(
                args.separated_by(just(',').padded())
                    .collect::<Vec<_>>()
                    .delimited_by(just('(').padded(), just(')').padded()),
            )
            .map(|(func, args)| Expression::Call { func, args })
    }
}

impl Parsable for Expression {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        recursive(|expr| {
            // Define the literal and identifier parsers
            let literal = Literal::parser().map(Expression::Literal).boxed();
            let identifier = Identifier::parser().map(Expression::Identifier).boxed();
            let unary = UnaryOperator::parser()
                .then(expr.clone())
                .map(|(op, expr)| Expression::UnaryExpr {
                    op,
                    expr: Box::new(expr),
                });
            let function = Self::function(expr);

            // Define the primary expression parser
            let primary = choice((function, literal, identifier, unary)).boxed();

            // Define the Pratt parser for binary expressions
            primary.clone().pratt((
                infix(
                    Associativity::Left(5),
                    BinaryOperator::multiplicative().boxed(),
                    |left, op, right| Expression::BinaryExpr {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                ),
                infix(
                    Associativity::Left(4),
                    BinaryOperator::additive().boxed(),
                    |left, op, right| Expression::BinaryExpr {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                ),
                infix(
                    Associativity::Left(3),
                    BinaryOperator::comparison().boxed(),
                    |left, op, right| Expression::BinaryExpr {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                ),
                infix(
                    Associativity::Left(2),
                    BinaryOperator::and().boxed(),
                    |left, op, right| Expression::BinaryExpr {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                ),
                infix(
                    Associativity::Left(1),
                    BinaryOperator::or_xor().boxed(),
                    |left, op, right| Expression::BinaryExpr {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                ),
            ))
        })
    }
}

impl Expression {
    /// Create a new unary expression, boxing the expression
    pub fn unary_expr(op: UnaryOperator, expr: Self) -> Self {
        Expression::UnaryExpr {
            op,
            expr: Box::new(expr),
        }
    }

    /// Create a new binary expression, boxing the left and right expressions
    pub fn binary_expr(lhs: Self, op: BinaryOperator, rhs: Self) -> Self {
        Expression::BinaryExpr {
            left: Box::new(lhs),
            op,
            right: Box::new(rhs),
        }
    }

    pub fn call(func: Identifier, args: Vec<Expression>) -> Self {
        Expression::Call { func, args }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{test_parser, Expect};

    #[test]
    fn test_parse_literal() {
        test_parser("42", Expression::Literal(Literal::Integer(42)));
    }

    #[test]
    fn test_parse_identifier() {
        test_parser("foobar", Expression::Identifier(Identifier::new("foobar")));
    }

    #[test]
    fn test_parse_unary_expression() {
        test_parser(
            "!foobar",
            Expression::unary_expr(
                UnaryOperator::Not,
                Expression::Identifier(Identifier::new("foobar")),
            ),
        );
    }

    #[test]
    fn test_addition() {
        test_parser(
            "10+10",
            Expression::binary_expr(
                Expression::Literal(Literal::Integer(10)),
                BinaryOperator::Add,
                Expression::Literal(Literal::Integer(10)),
            ),
        );
    }

    // Add more tests as needed
    #[test]
    fn test_multiplication() {
        test_parser(
            "5*3",
            Expression::binary_expr(
                Expression::Literal(Literal::Integer(5)),
                BinaryOperator::Multiply,
                Expression::Literal(Literal::Integer(3)),
            ),
        );
    }

    #[test]
    fn test_complex_expression() {
        test_parser(
            "5+3*2",
            Expression::binary_expr(
                Expression::Literal(Literal::Integer(5)),
                BinaryOperator::Add,
                Expression::binary_expr(
                    Expression::Literal(Literal::Integer(3)),
                    BinaryOperator::Multiply,
                    Expression::Literal(Literal::Integer(2)),
                ),
            ),
        );
    }

    #[test]
    fn test_comparison() {
        test_parser(
            "10 == 10",
            Expression::binary_expr(
                Expression::Literal(Literal::Integer(10)),
                BinaryOperator::Equals,
                Expression::Literal(Literal::Integer(10)),
            ),
        );
    }

    #[test]
    fn test_logical_and() {
        test_parser(
            "true and false",
            Expression::binary_expr(
                Expression::Literal(Literal::Boolean(true)),
                BinaryOperator::And,
                Expression::Literal(Literal::Boolean(false)),
            ),
        );
    }

    #[test]
    fn test_logical_or() {
        test_parser(
            "true or false",
            Expression::binary_expr(
                Expression::Literal(Literal::Boolean(true)),
                BinaryOperator::Or,
                Expression::Literal(Literal::Boolean(false)),
            ),
        );
    }

    #[test]
    fn test_functions() {
        test_parser(
            "foobar()",
            Expression::call(Identifier::new("foobar"), vec![]),
        );
        test_parser(
            "foobar(a, b)",
            Expression::call(
                Identifier::new("foobar"),
                vec![
                    Expression::Identifier(Identifier::new("a")),
                    Expression::Identifier(Identifier::new("b")),
                ],
            ),
        );
        test_parser(
            "foobar(42)",
            Expression::call(
                Identifier::new("foobar"),
                vec![Expression::Literal(Literal::Integer(42))],
            ),
        );
    }

    impl From<Expression> for Expect<Expression> {
        fn from(value: Expression) -> Self {
            Expect::Something(value)
        }
    }
}
