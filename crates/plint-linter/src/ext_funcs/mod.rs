mod count;
mod logic;
mod string;

use plint_lang::Interpreter;

pub(super) fn add_ext_funcs(interpreter: &mut Interpreter) {
    count::add_funcs(interpreter);
    logic::add_funcs(interpreter);
    string::add_funcs(interpreter);
}
