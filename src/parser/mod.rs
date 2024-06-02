use crate::parser::literal::Literal;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::prelude::just;
use chumsky::{extra, Parser};
use std::fmt::Debug;

mod binary_operator;
mod comment;
mod expression;
mod identifier;
mod literal;
mod quantifier;
mod statement;
mod unary_operator;

pub(crate) use binary_operator::*;
pub(crate) use expression::*;
pub(crate) use identifier::*;
pub(crate) use literal::*;
pub(crate) use quantifier::*;
pub(crate) use statement::*;
pub(crate) use unary_operator::*;

type ParsableError<'src> = extra::Err<Rich<'src, char>>;

pub(crate) trait Parsable {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>>
    where
        Self: Sized;
}

impl Parsable for () {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>>
    where
        Self: Sized,
    {
        just("").to(())
    }
}

pub(crate) enum Expect<T> {
    Something(T),
    Error(String),
}

impl<T> From<&str> for Expect<T> {
    fn from(value: &str) -> Self {
        Expect::Error(value.to_string())
    }
}

pub(crate) fn test_parser<K: Parsable + Debug + PartialEq, T: Into<Expect<K>>>(
    input: &str,
    expected: T,
) {
    let (out, errors) = K::parser().parse(input).into_output_errors();
    let mut reasons = vec![];
    errors.iter().for_each(|e| {
        reasons.push(e.reason().to_string());
        Report::build(ReportKind::Error, (), e.span().start)
            .with_message(e.to_string())
            .with_label(
                Label::new(e.span().into_range())
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(input))
            .unwrap()
    });
    match expected.into() {
        Expect::Something(expected) => {
            assert_eq!(out.unwrap(), expected, "input: {:?}", input);
        }
        Expect::Error(expected) => {
            if reasons.iter().find(|r| r.contains(&expected)).is_none() {
                panic!("expected error: {:?} got: {:?}", expected, reasons)
            }
        }
    }
}
