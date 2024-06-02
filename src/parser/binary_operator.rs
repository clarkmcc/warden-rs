use crate::parser::{Parsable, ParsableError};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    Xor,
    Contains,
    In,
    Matches,
    NotMatches,
    Is,
    IsNot,
}

impl Parsable for BinaryOperator {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        choice((
            Self::additive(),
            Self::multiplicative(),
            Self::comparison(),
            Self::and(),
            Self::or_xor(),
        ))
    }
}

impl BinaryOperator {
    pub(crate) fn multiplicative<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>>
    {
        choice((
            just('*').to(BinaryOperator::Multiply),
            just('/').to(BinaryOperator::Divide),
            just('%').to(BinaryOperator::Modulus),
        ))
        .padded()
    }

    pub(crate) fn additive<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        choice((
            just('+').to(BinaryOperator::Add),
            just('-').to(BinaryOperator::Subtract),
        ))
        .padded()
    }

    pub(crate) fn comparison<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        choice((
            just("==").to(BinaryOperator::Equals),
            just("!=").to(BinaryOperator::NotEquals),
            just("<=").to(BinaryOperator::LessThanOrEqual),
            just('<').to(BinaryOperator::LessThan),
            just(">=").to(BinaryOperator::GreaterThanOrEqual),
            just('>').to(BinaryOperator::GreaterThan),
            just("is not").to(BinaryOperator::IsNot),
            just("is").to(BinaryOperator::Is),
            just("matches").to(BinaryOperator::Matches),
            just("not matches").to(BinaryOperator::NotMatches),
            just("contains").to(BinaryOperator::Contains),
            just("in").to(BinaryOperator::In),
        ))
        .padded()
    }

    pub(crate) fn and<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        just("and").to(BinaryOperator::And).padded()
    }

    pub(crate) fn or_xor<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        choice((
            just("or").to(BinaryOperator::Or),
            just("xor").to(BinaryOperator::Xor),
        ))
        .padded()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{test_parser, BinaryOperator, Expect};

    #[test]
    fn test_parse() {
        test_parser("+", BinaryOperator::Add);
        test_parser("-", BinaryOperator::Subtract);
        test_parser("*", BinaryOperator::Multiply);
        test_parser("/", BinaryOperator::Divide);
        test_parser("%", BinaryOperator::Modulus);
        test_parser("==", BinaryOperator::Equals);
        test_parser("!=", BinaryOperator::NotEquals);
        test_parser("<", BinaryOperator::LessThan);
        test_parser(">", BinaryOperator::GreaterThan);
        test_parser("<=", BinaryOperator::LessThanOrEqual);
        test_parser(">=", BinaryOperator::GreaterThanOrEqual);
        test_parser("and", BinaryOperator::And);
        test_parser("or", BinaryOperator::Or);
        test_parser("xor", BinaryOperator::Xor);
        test_parser("contains", BinaryOperator::Contains);
        test_parser("in", BinaryOperator::In);
        test_parser("not matches", BinaryOperator::NotMatches);
        test_parser("matches", BinaryOperator::Matches);
        test_parser("is not", BinaryOperator::IsNot);
        test_parser("is", BinaryOperator::Is);
    }

    impl From<BinaryOperator> for Expect<BinaryOperator> {
        fn from(value: BinaryOperator) -> Self {
            Expect::Something(value)
        }
    }
}
