use chumsky::prelude::*;

// fn comment<'src>() -> impl Parser<'src, &'src str, (), extra::Err<Rich<'src, char>>> {
//     let single_line = just("//")
//         .or(just("#"))
//         .then(take_until(text::newline()).ignored());
//
//     let multi_line = just("/*").then(take_until(just("*/"))).ignored();
//
//     single_line.or(multi_line).ignored()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn test_comment() {}
}
