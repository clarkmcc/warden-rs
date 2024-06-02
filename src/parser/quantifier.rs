use crate::parser::{Parsable, ParsableError};
use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum QuantifierType {
    All,
    Any,
    Filter,
    Map,
}

impl Parsable for QuantifierType {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        choice((
            text::keyword("all").to(QuantifierType::All),
            text::keyword("any").to(QuantifierType::Any),
            text::keyword("filter").to(QuantifierType::Filter),
            text::keyword("map").to(QuantifierType::Map),
        ))
    }
}
