use crate::Condition;
use serde::{
    Deserialize,
    de::{self, Visitor},
};

struct ConditionVisitor;

impl<'de> Visitor<'de> for ConditionVisitor {
    type Value = Condition;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid condition")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        if let Ok(Some(key)) = map.next_key::<String>() {
            match key.as_str() {
                "eq" => {
                    return Ok(Condition::Eq(map.next_value::<crate::Value>()?));
                }
                "neq" => {
                    return Ok(Condition::Neq(map.next_value::<crate::Value>()?));
                }
                "gt" => {
                    return Ok(Condition::Gt(map.next_value::<crate::Value>()?));
                }
                "gte" => {
                    return Ok(Condition::Gte(map.next_value::<crate::Value>()?));
                }
                "lt" => {
                    return Ok(Condition::Lt(map.next_value::<crate::Value>()?));
                }
                "lte" => {
                    return Ok(Condition::Lte(map.next_value::<crate::Value>()?));
                }
                "and" => {
                    return Ok(Condition::And(map.next_value::<Vec<Condition>>()?));
                }
                "or" => {
                    return Ok(Condition::Or(map.next_value::<Vec<Condition>>()?));
                }
                "not" => {
                    return Ok(Condition::Not(Box::new(map.next_value::<Condition>()?)));
                }
                _ => {
                    return Err(de::Error::unknown_field(
                        &key,
                        &["eq", "neq", "gt", "gte", "le", "lte", "and", "or", "not"],
                    ));
                }
            }
        }

        Err(de::Error::invalid_length(
            0,
            &"a condition with a single key",
        ))
    }
}

impl<'de> Deserialize<'de> for Condition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(ConditionVisitor)
    }
}
