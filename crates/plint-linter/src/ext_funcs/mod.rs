mod calc;
mod count;
mod logic;
mod md;
mod string;

use plint_lang::Interpreter;

pub(super) fn add_ext_funcs(interpreter: &mut Interpreter) {
    calc::add_funcs(intepreter);
    count::add_funcs(interpreter);
    logic::add_funcs(interpreter);
    string::add_funcs(interpreter);
    md::add_funcs(interpreter);
}
