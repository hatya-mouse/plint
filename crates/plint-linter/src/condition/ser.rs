use crate::Condition;
use serde::{Serialize, ser::SerializeMap};

impl Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Condition::Eq(value) => serialize_single_field(serializer, "eq", value),
            Condition::Neq(value) => serialize_single_field(serializer, "neq", value),
            Condition::Gt(value) => serialize_single_field(serializer, "gt", value),
            Condition::Gte(value) => serialize_single_field(serializer, "gte", value),
            Condition::Lt(value) => serialize_single_field(serializer, "lt", value),
            Condition::Lte(value) => serialize_single_field(serializer, "lte", value),
            Condition::And(conds) => serialize_single_field(serializer, "and", conds),
            Condition::Or(conds) => serialize_single_field(serializer, "or", conds),
            Condition::Not(cond) => serialize_single_field(serializer, "not", cond),
        }
    }
}

fn serialize_single_field<S, K, V>(serializer: S, key: &K, value: &V) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    K: ?Sized + Serialize,
    V: ?Sized + Serialize,
{
    let mut map = serializer.serialize_map(Some(1))?;
    map.serialize_entry(key, value)?;
    map.end()
}
