use crate::parser::{Parsable, ParsableError};
use chumsky::prelude::*;
use chumsky::Parser;

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    IsEmpty,
    IsNotEmpty,
    IsDefined,
    IsNotDefined,
}

impl Parsable for UnaryOperator {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        choice((
            just("+").to(UnaryOperator::Plus),
            just("-").to(UnaryOperator::Minus),
            just("!").to(UnaryOperator::Not),
            just("is empty").to(UnaryOperator::IsEmpty),
            just("is not empty").to(UnaryOperator::IsNotEmpty),
            just("is defined").to(UnaryOperator::IsDefined),
            just("is not defined").to(UnaryOperator::IsNotDefined),
        ))
    }
}
