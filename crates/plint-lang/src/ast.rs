pub(super) enum Expression {
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Variable {
        name: String,
    },
    For {
        variable: String,
        iterable: Box<Expression>,
        body: Vec<Expression>,
    },
    If {
        main: IfArm,
        else_ifs: Vec<IfArm>,
        else_body: Vec<Expression>,
    },
}

pub(super) struct IfArm {
    pub(crate) condition: Box<Expression>,
    pub(crate) body: Vec<Expression>,
}
