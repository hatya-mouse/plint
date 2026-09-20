use crate::Value;
use nom::{
    IResult, Parser,
    branch::alt,
    character::streaming::{char, one_of},
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::{preceded, terminated},
};

fn literal(input: &str) -> IResult<&str, Value> {
    alt((number, string)).parse(input)
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

fn string(input: &str) -> IResult<&str, Value> {}
