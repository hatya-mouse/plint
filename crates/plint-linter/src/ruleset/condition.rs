#[derive(Debug)]
pub enum Condition {
    Eq(crate::Value),
    Neq(crate::Value),
    Gt(crate::Value),
    Gte(crate::Value),
    Lt(crate::Value),
    Lte(crate::Value),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub(crate) fn evaluate(&self, a: &crate::Value) -> bool {
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
    a: &crate::Value,
    b: &crate::Value,
    cmp_str: impl FnOnce(&str, &str) -> bool,
    cmp_i64: impl FnOnce(i64, i64) -> bool,
    cmp_f64: impl FnOnce(f64, f64) -> bool,
    cmp_bool: impl FnOnce(bool, bool) -> bool,
) -> bool {
    match a {
        crate::Value::String(a_str) => cmp_str(a_str, &b.force_string()),
        crate::Value::Integer(a_int) => b.try_i64().is_some_and(|b_int| cmp_i64(*a_int, b_int)),
        crate::Value::Float(a_float) => b
            .try_f64()
            .is_some_and(|b_float| cmp_f64(*a_float, b_float)),
        crate::Value::Boolean(a_bool) => {
            b.try_bool().is_some_and(|b_bool| cmp_bool(*a_bool, b_bool))
        }
    }
}
