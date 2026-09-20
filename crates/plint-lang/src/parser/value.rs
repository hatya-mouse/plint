use crate::{Value, ast::Expr};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::streaming::escaped_transform,
    character::streaming::{char, none_of, one_of},
    combinator::{opt, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
};

pub(super) fn literal(input: &str) -> IResult<&str, Expr> {
    alt((number, string)).map(Expr::Literal).parse(input)
}

fn number(input: &str) -> IResult<&str, Value> {
    alt((
        recognize((
            char('.'),
            decimal,
            opt((one_of("eE"), opt(one_of("+-")), decimal)),
        )),
        recognize((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        )),
        recognize((decimal, char('.'), opt(decimal))),
        recognize(decimal),
    ))
    .map_res(|str| str.parse::<f64>())
    .map(Value::Number)
    .parse(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
}

fn string(input: &str) -> IResult<&str, Value> {
    delimited(
        char('"'),
        escaped_transform(
            none_of("\\\""),
            '\\',
            alt((
                value('\\', char('\\')),
                value('"', char('"')),
                value('\n', char('n')),
            )),
        ),
        char('"'),
    )
    .map(Value::String)
    .parse(input)
}
