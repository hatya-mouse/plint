use std::range::Range;

pub(super) enum Expr {
    For {
        loop_var: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    If {
        main: IfArm,
        else_ifs: Vec<IfArm>,
        else_body: Vec<Expr>,
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
    Number(f64),
    String(String),
    Bool(bool),
    Null,
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
