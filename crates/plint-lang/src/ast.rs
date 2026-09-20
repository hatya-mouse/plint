use std::{fmt::Display, range::Range};

pub(super) enum Expr {
    For {
        loop_var: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    If {
        main: IfArm,
        else_ifs: Vec<IfArm>,
        else_body: Option<Vec<Expr>>,
    },
    Literal(Value),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Variable {
        name: String,
    },
}

pub(super) struct IfArm {
    pub(crate) condition: Box<Expr>,
    pub(crate) body: Vec<Expr>,
}

pub enum Value {
    List(Vec<Value>),
    Match(Range<usize>),
    Integer(i64),
    String(String),
    Bool(bool),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::List(list) => write!(
                f,
                "[{}]",
                list.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Match(range) => write!(f, "Match({:?})", range),
            Value::Integer(integer) => write!(f, "{}", integer),
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::Bool(boolean) => write!(f, "{}", boolean),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        !self.is_null()
    }
}
