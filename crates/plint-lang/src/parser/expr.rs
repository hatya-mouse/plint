use crate::{
    ast::{Expr, IfArm},
    parser::value::literal,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag,
    character::streaming::{alpha1, alphanumeric1, multispace0, space0, space1},
    combinator::{opt, recognize},
    multi::{many0, many0_count, separated_list0},
    sequence::{delimited, pair, preceded, terminated},
};

// --- EXPRESSION ---

fn expr(input: &str) -> IResult<&str, Expr> {
    // Place the variable parser at the end to avoid matching keywords,
    // function names and assign l-values as variables.
    alt((
        parenthesized,
        for_loop,
        if_expr,
        literal,
        func_call,
        assign,
        variable,
    ))
    .parse(input)
}

pub(crate) fn exprs(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(delimited(multispace0, expr, multispace0)).parse(input)
}

// --- IDENTIFIER ---

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

// --- PARENTHESIZED ---

fn parenthesized(input: &str) -> IResult<&str, Expr> {
    delimited(tag("("), exprs, tag(")"))
        .map(Expr::Parenthesized)
        .parse(input)
}

// --- FOR ---

fn for_loop(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("for").parse(input)?;
    let (input, loop_var) = delimited(space1, identifier, space1).parse(input)?;
    let (input, _) = tag("in").parse(input)?;
    let (input, iterable) = delimited(space1, expr, space0).parse(input)?;
    let (input, body) = delimited(tag("("), exprs, tag(")")).parse(input)?;

    let for_loop = Expr::For {
        loop_var: loop_var.to_string(),
        iterable: Box::new(iterable),
        body,
    };

    Ok((input, for_loop))
}

// --- IF ---

fn if_expr(input: &str) -> IResult<&str, Expr> {
    let (input, main) = if_arm.parse(input)?;
    let (input, else_ifs) = many0(preceded(
        delimited(multispace0, tag("else"), multispace0),
        if_arm,
    ))
    .parse(input)?;
    let (input, else_body) = opt(preceded(
        delimited(multispace0, tag("else"), multispace0),
        delimited(tag("("), exprs, tag(")")),
    ))
    .parse(input)?;

    let if_expr = Expr::If {
        main,
        else_ifs,
        else_body,
    };

    Ok((input, if_expr))
}

fn if_arm(input: &str) -> IResult<&str, IfArm> {
    let (input, _) = tag("if").parse(input)?;
    let (input, condition) = delimited(space1, expr, multispace0).parse(input)?;
    let (input, body) = delimited(tag("("), exprs, tag(")")).parse(input)?;

    let if_arm = IfArm {
        condition: Box::new(condition),
        body,
    };

    Ok((input, if_arm))
}

// --- FUNCTION CALL ---

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

// --- ASSIGN ---

fn assign(input: &str) -> IResult<&str, Expr> {
    let (input, name) = identifier.parse(input)?;
    let (input, _) = delimited(space0, tag("="), space0).parse(input)?;
    let (input, value) = expr.parse(input)?;

    let assign = Expr::Assign {
        name: name.to_string(),
        value: Box::new(value),
    };

    Ok((input, assign))
}

// --- VARIABLE ---

fn variable(input: &str) -> IResult<&str, Expr> {
    identifier
        .map(|name| Expr::Variable {
            name: name.to_string(),
        })
        .parse(input)
}
