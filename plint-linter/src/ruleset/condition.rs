use crate::checker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Condition {
    #[serde(rename = "eq")]
    Eq(yaml_serde::Value),
    #[serde(rename = "neq")]
    Neq(yaml_serde::Value),
    #[serde(rename = "gt")]
    Gt(yaml_serde::Value),
    #[serde(rename = "gte")]
    Gte(yaml_serde::Value),
    #[serde(rename = "lt")]
    Lt(yaml_serde::Value),
    #[serde(rename = "lte")]
    Lte(yaml_serde::Value),
    #[serde(rename = "and")]
    And(Vec<Condition>),
    #[serde(rename = "or")]
    Or(Vec<Condition>),
    #[serde(rename = "not")]
    Not(Box<Condition>),
}

impl Condition {
    pub(crate) fn evaluate(&self, a: &checker::Value) -> bool {
        match self {
            Condition::Eq(b) => comp_value(
                a,
                b,
                |a, b| a == b,
                |a, b| a == b,
                |a, b| a == b,
                |a, b| a == b,
            ),
            Condition::Neq(b) => comp_value(
                a,
                b,
                |a, b| a != b,
                |a, b| a != b,
                |a, b| a != b,
                |a, b| a != b,
            ),
            Condition::Gt(b) => comp_value(
                a,
                b,
                |_, _| false,
                |a, b| a > b,
                |a, b| a > b,
                |a, b| a && !b,
            ),
            Condition::Gte(b) => comp_value(
                a,
                b,
                |_, _| false,
                |a, b| a >= b,
                |a, b| a >= b,
                |a, b| a || !b,
            ),
            Condition::Lt(b) => comp_value(
                a,
                b,
                |_, _| false,
                |a, b| a < b,
                |a, b| a < b,
                |a, b| !a && b,
            ),
            Condition::Lte(b) => comp_value(
                a,
                b,
                |_, _| false,
                |a, b| a <= b,
                |a, b| a <= b,
                |a, b| !a || b,
            ),
            Condition::And(conds) => conds.iter().all(|cond| cond.evaluate(a)),
            Condition::Or(conds) => conds.iter().any(|cond| cond.evaluate(a)),
            Condition::Not(cond) => !cond.evaluate(a),
        }
    }
}

fn comp_value(
    a: &checker::Value,
    b: &yaml_serde::Value,
    cmp_str: impl FnOnce(&str, &str) -> bool,
    cmp_i64: impl FnOnce(i64, i64) -> bool,
    cmp_f64: impl FnOnce(f64, f64) -> bool,
    cmp_bool: impl FnOnce(bool, bool) -> bool,
) -> bool {
    match b {
        yaml_serde::Value::String(b_str) => cmp_str(&a.into_string(), b_str),
        yaml_serde::Value::Number(b_n) => {
            if b_n.is_u64() || b_n.is_i64() {
                a.into_i64()
                    .is_some_and(|a_int| b_n.as_i64().is_some_and(|b_int| cmp_i64(a_int, b_int)))
            } else if b_n.is_f64() {
                a.into_f64()
                    .is_some_and(|a_int| b_n.as_f64().is_some_and(|b_int| cmp_f64(a_int, b_int)))
            } else {
                false
            }
        }
        yaml_serde::Value::Bool(b_bool) => a
            .into_bool()
            .is_some_and(|a_bool| cmp_bool(a_bool, *b_bool)),
        _ => false,
    }
}
