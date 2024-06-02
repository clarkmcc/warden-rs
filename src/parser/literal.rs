use crate::parser::Parsable;
use chumsky::prelude::*;
use std::sync::Arc;
use strum_macros::EnumString;

#[derive(PartialEq, Debug, Clone, EnumString)]
pub enum Literal {
    Null,
    Undefined,
    Integer(i64),
    Float(f64),
    String(Arc<String>),
    Boolean(bool),
}

impl Parsable for Literal {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Rich<'src, char>>> {
        // Floating-point Literals
        // A floating-point literal is a decimal representation of a floating-point constant. It has
        // an integer part, a decimal point, a fractional part, and an exponent part. The integer and
        // fractional part comprise decimal digits; the exponent part is an e or E followed by an
        // optionally signed decimal exponent. One of the integer part or the fractional part may be
        // elided; one of the decimal point or the exponent may be elided.
        //
        // Floating-point numbers are IEEE-754 64-bit floating numbers.
        //
        // float_lit = decimals "." [ decimals ] [ exponent ] |
        //             decimals exponent |
        //             "." decimals [ exponent ] .
        // decimals  = decimal_digit { decimal_digit } .
        // exponent  = ( "e" | "E" ) [ "+" | "-" ] decimals .
        //
        //  0.
        //  72.40
        //  072.40  // == 72.40
        //  2.71828
        //  1.e+0
        //  6.67428e-11
        //  1E6
        //  .25
        //  .12345E+5
        let exponent = just('e')
            .or(just('E'))
            .then(just('+').or(just('-')).or_not())
            .then(text::digits(10));
        let float = choice((
            text::digits(10)
                .then_ignore(just('.'))
                .then(text::digits(10).or_not())
                .then(exponent.or_not())
                .to_slice()
                .from_str()
                .unwrapped()
                .map(|v| Literal::Float(v)),
            text::digits(10)
                .then(exponent)
                .to_slice()
                .from_str()
                .unwrapped()
                .map(|v| Literal::Float(v)),
            just('.')
                .ignore_then(text::digits(10))
                .then(exponent.or_not())
                .to_slice()
                .from_str()
                .unwrapped()
                .map(|v| Literal::Float(v)),
        ));

        // Integer Literals
        // An integer literal is a sequence of digits representing an integer constant.
        // An optional prefix sets a non-decimal base: 0 for octal, 0x or 0X for hexadecimal.
        // In hexadecimal literals, letters a-f and A-F represents values 10 through 15.
        //
        // Integers are signed 64-bit values (-9223372036854775808 to 9223372036854775807).
        //
        // int_lit     = decimal_lit | octal_lit | hex_lit .
        // decimal_lit = ( "1" … "9" ) { decimal_digit } .
        // octal_lit   = "0" { octal_digit } .
        // hex_lit     = "0" ( "x" | "X" ) hex_digit { hex_digit } .
        //
        // 42
        // 0600
        // 0xBadFace
        // 170141183460469231731687303715884105727
        let decimal = text::int(10).map(|s: &str| Literal::Integer(s.parse().unwrap()));
        let octal = just('0').then(text::digits(8)).to_slice().map(|s: &str| {
            if s == "0" {
                Literal::Integer(0)
            } else {
                i64::from_str_radix(s, 8).map(Literal::Integer).unwrap()
            }
        });
        let hex = just("0x")
            .or(just("0X"))
            .ignore_then(text::digits(16).to_slice())
            .map(|s: &str| i64::from_str_radix(s, 16).map(Literal::Integer).unwrap());
        let integer = choice((hex, octal, decimal));

        // Boolean Literals
        let boolean = choice((
            just::<&str, &str, extra::Err<Rich<'src, char>>>("true")
                .map(|_| Literal::Boolean(true)),
            just::<&str, &str, extra::Err<Rich<'src, char>>>("false")
                .map(|_| Literal::Boolean(false)),
        ));

        // String Literals
        // let escape_sequence = choice((
        //     just("\\a").map(|_| '\x07'),
        //     just("\\b").map(|_| '\x08'),
        //     just("\\f").map(|_| '\x0C'),
        //     just("\\n").map(|_| '\x0A'),
        //     just("\\r").map(|_| '\x0D'),
        //     just("\\t").map(|_| '\x09'),
        //     just("\\v").map(|_| '\x0B'),
        //     just("\\\\").map(|_| '\\'),
        //     just("\\\"").map(|_| '"'),
        //     just("\\x").ignore_then(
        //         text::digits(16)
        //             .exactly(2)
        //             .collect::<String>()
        //             .validate(|s, span, emitter| {
        //                 if u8::from_str_radix(&s, 16).is_err() {
        //                     emitter.emit(Rich::custom(
        //                         span,
        //                         format!("Invalid hex byte value: \\x{}", s),
        //                     ));
        //                 }
        //                 s // Return the string so it can be used in the next map
        //             })
        //             .map(|s| u8::from_str_radix(&s, 16).unwrap() as char),
        //     ),
        //     just("\\u").ignore_then(
        //         text::digits(16)
        //             .exactly(4)
        //             .collect::<String>()
        //             .validate(|s, span, emitter| {
        //                 if u16::from_str_radix(&s, 16).is_err() {
        //                     emitter.emit(Rich::custom(
        //                         span,
        //                         format!("Invalid Unicode value: \\u{}", s),
        //                     ));
        //                 }
        //                 s // Return the string so it can be used in the next map
        //             })
        //             .map(|s| char::from_u32(u32::from_str_radix(&s, 16).unwrap()).unwrap()),
        //     ),
        //     just("\\U").ignore_then(
        //         text::digits(16)
        //             .exactly(8)
        //             .collect::<String>()
        //             .validate(|s, span, emitter| {
        //                 if let Ok(value) = u32::from_str_radix(&s, 16) {
        //                     if char::from_u32(value).is_none() {
        //                         emitter.emit(Rich::custom(
        //                             span,
        //                             format!("Invalid Unicode value: \\U{}", s),
        //                         ));
        //                     }
        //                 } else {
        //                     emitter.emit(Rich::custom(
        //                         span,
        //                         format!("Invalid Unicode value: \\U{}", s),
        //                     ));
        //                 }
        //                 s // Return the string so it can be used in the next map
        //             })
        //             .map(|s| char::from_u32(u32::from_str_radix(&s, 16).unwrap()).unwrap()),
        //     ),
        //     just("\\").ignore_then(
        //         text::digits(8)
        //             .exactly(3)
        //             .collect::<String>()
        //             .validate(|s, span, emitter| {
        //                 if u8::from_str_radix(&s, 8).is_err() {
        //                     emitter.emit(Rich::custom(
        //                         span,
        //                         format!("Invalid octal byte value: \\{}", s),
        //                     ));
        //                 }
        //                 s // Return the string so it can be used in the next map
        //             })
        //             .map(|s| u8::from_str_radix(&s, 8).unwrap() as char),
        //     ),
        // ));

        let string = just('"')
            .ignore_then(none_of('"').repeated().collect::<String>())
            .then_ignore(just('"'))
            .map(|v| Literal::String(Arc::new(v)));

        let undefined = just::<&str, &str, extra::Err<Rich<'src, char>>>("undefined")
            .map(|_| Literal::Undefined);
        let null = just::<&str, &str, extra::Err<Rich<'src, char>>>("null").map(|_| Literal::Null);

        choice((float, integer, string, boolean, undefined, null))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{test_parser, Expect};

    #[test]
    fn test_parse_float() {
        test_parser("123.45", Literal::Float(123.45));
        test_parser("1.e+0", Literal::Float(1.0));
        test_parser("6.67428e-11", Literal::Float(6.67428e-11));
        test_parser("1E6", Literal::Float(1e6));
        test_parser(".25", Literal::Float(0.25));
        test_parser(".12345E+5", Literal::Float(12345.0));
    }

    #[test]
    fn test_parse_integer() {
        test_parser("123", Literal::Integer(123));
        test_parser("0", Literal::Integer(0));
        // Octal
        test_parser("076", Literal::Integer(62));
        test_parser::<Literal, &str>("099", "found end of input");
        // Hexadecimal
        test_parser("0x1A3F", Literal::Integer(0x1A3F));
        test_parser::<Literal, &str>("0x9X", "found X expected end of input");
        test_parser("0X1A3F", Literal::Integer(0x1A3F));
    }

    #[test]
    fn test_parse_string() {
        test_parser(r#""hello""#, Literal::String(Arc::new("hello".to_string())));
        test_parser(r#""world""#, Literal::String(Arc::new("world".to_string())));
        test_parser(r#""12345""#, Literal::String(Arc::new("12345".to_string())));
        test_parser(r#""""#, Literal::String(Arc::new("".to_string())));
    }

    #[test]
    fn test_parse_boolean() {
        test_parser("true", Literal::Boolean(true));
        test_parser("false", Literal::Boolean(false));
    }

    impl From<Literal> for Expect<Literal> {
        fn from(value: Literal) -> Self {
            Expect::Something(value)
        }
    }
}
