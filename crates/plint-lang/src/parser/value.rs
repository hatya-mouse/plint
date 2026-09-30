//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{Value, ast::Expr};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::complete::{char, none_of, one_of},
    combinator::{opt, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, terminated},
};

pub(super) fn literal(input: &str) -> IResult<&str, Expr> {
    alt((integer, string, null)).map(Expr::Literal).parse(input)
}

// --- INTEGER ---

// fn number(input: &str) -> IResult<&str, Value> {
//     alt((
//         recognize((
//             char('.'),
//             decimal,
//             opt((one_of("eE"), opt(one_of("+-")), decimal)),
//         )),
//         recognize((
//             decimal,
//             opt(preceded(char('.'), decimal)),
//             one_of("eE"),
//             opt(one_of("+-")),
//             decimal,
//         )),
//         recognize((decimal, char('.'), opt(decimal))),
//         recognize(decimal),
//     ))
//     .map_res(|str| str.parse::<f64>())
//     .map(Value::Number)
//     .parse(input)
// }

fn integer(input: &str) -> IResult<&str, Value> {
    recognize((opt(tag("-")), decimal))
        .map_res(|string| string.parse::<i64>())
        .map(Value::Integer)
        .parse(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
}

// --- STRING ---

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

// --- NULL ---

fn null(input: &str) -> IResult<&str, Value> {
    tag("null").map(|_| Value::Null).parse(input)
}
