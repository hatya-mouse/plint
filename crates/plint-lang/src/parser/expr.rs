use crate::{
    ast::{Expr, IfArm},
    parser::value::literal,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::streaming::{alpha1, alphanumeric1, multispace0, space0, space1},
    combinator::recognize,
    multi::{many0, many0_count, separated_list0},
    sequence::{delimited, pair, terminated},
};

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((for_loop, if_expr, literal, func_call, variable)).parse(input)
}

fn exprs(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(delimited(multispace0, expr, multispace0)).parse(input)
}

fn for_loop(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("for").parse(input)?;
    let (input, loop_var) = delimited(space1, identifier, space1).parse(input)?;
    let (input, _) = tag("in").parse(input)?;
    let (input, iterable) = delimited(space1, expr, space0).parse(input)?;
    let (input, body) = delimited(tag("{"), exprs, tag("}")).parse(input)?;

    let for_loop = Expr::For {
        loop_var: loop_var.to_string(),
        iterable: Box::new(iterable),
        body,
    };

    Ok((input, for_loop))
}

fn if_expr(input: &str) -> IResult<&str, Expr> {}

fn if_arm(input: &str) -> IResult<&str, IfArm> {}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

fn func_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = identifier.parse(input)?;
    let (input, args) = delimited(
        tag("("),
        terminated(
            separated_list0(delimited(space0, tag(","), space0), expr),
            delimited(space0, tag(","), space0),
        ),
        tag(")"),
    )
    .parse(input)?;

    let func_call = Expr::FunctionCall {
        name: name.to_string(),
        args,
    };

    Ok((input, func_call))
}

fn variable(input: &str) -> IResult<&str, Expr> {
    identifier
        .map(|name| Expr::Variable {
            name: name.to_string(),
        })
        .parse(input)
}
