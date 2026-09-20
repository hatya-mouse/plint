use nom::{
    IResult, Parser,
    branch::alt,
    character::streaming::{char, one_of},
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::{preceded, terminated},
};

fn number(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(decimal),
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
    ))
    .parse(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
}
