use crate::parser::{Parsable, ParsableError};
use chumsky::{text, Parser};

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn new<T: Into<String>>(s: T) -> Self {
        Identifier(s.into())
    }
}

impl Parsable for Identifier {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, ParsableError<'src>> {
        text::ident().map(|s: &str| Identifier(s.to_string()))
    }
}
