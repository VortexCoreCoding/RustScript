pub mod commands;
pub mod dict;
pub mod interpreter;
pub mod parser;
pub mod stack;
pub mod types;

pub use commands::{
    abs, add, and, begin, ceiling, clear, copy, count, dict, div, dup, end, eq, eq_eq_print,
    eq_print, exch, floor, ge, get, getinterval, gt, idiv, le, length, lt, maxlength, mul, ne, neg,
    not, or, pop, print, ps_def, ps_false, ps_for, ps_help, ps_if, ps_ifelse, ps_mod, ps_repeat,
    ps_true, putinterval, quit, round, sqrt, sub,
};
pub use interpreter::Interpreter;
pub use parser::parse;
pub use types::PSValue;

pub fn exec_proc(interp: &mut interpreter::Interpreter, proc: types::PSValue) {
    if let types::PSValue::Proc { body, env } = proc {
        match interp.scope {
            interpreter::ScopeMode::Dynamic => {
                execute(interp, body);
            }
            interpreter::ScopeMode::Lexical => {
                if let Some(captured_env) = env {
                    let old_stack = interp.dict_stack.snapshot();
                    interp.dict_stack.replace(captured_env);
                    execute(interp, body);
                    interp.dict_stack.replace(old_stack);
                }
            }
        }
    }
}

pub fn execute(interp: &mut interpreter::Interpreter, program: Vec<types::PSValue>) {
    for token in program {
        match token {
            types::PSValue::Int(_)
            | types::PSValue::Bool(_)
            | types::PSValue::Str(_)
            | types::PSValue::Float(_) => {
                interp.op_stack.push(token);
            }
            types::PSValue::Name(_) => interp.op_stack.push(token),
            types::PSValue::Proc { body, env: _ } => {
                let proc_val = {
                    let captured: Vec<(std::collections::HashMap<String, PSValue>, i32)> =
                        interp.dict_stack.snapshot();
                    types::PSValue::Proc {
                        body,
                        env: Some(captured),
                    }
                };
                interp.op_stack.push(proc_val);
            }
            types::PSValue::Literal(name) => match name.as_str() {
                "add" => commands::add(interp),
                "sub" => commands::sub(interp),
                "mul" => commands::mul(interp),
                "div" => commands::div(interp),
                "idiv" => commands::idiv(interp),
                "mod" => commands::ps_mod(interp),
                "abs" => commands::abs(interp),
                "neg" => commands::neg(interp),
                "ceiling" => commands::ceiling(interp),
                "floor" => commands::floor(interp),
                "round" => commands::round(interp),
                "sqrt" => commands::sqrt(interp),
                "exch" => commands::exch(interp),
                "pop" => commands::pop(interp),
                "copy" => commands::copy(interp),
                "dup" => commands::dup(interp),
                "clear" => commands::clear(interp),
                "count" => commands::count(interp),
                "dict" => commands::dict(interp),
                "length" => commands::length(interp),
                "maxlength" => commands::maxlength(interp),
                "begin" => commands::begin(interp),
                "end" => commands::end(interp),
                "def" => commands::ps_def(interp),
                "eq" => commands::eq(interp),
                "ne" => commands::ne(interp),
                "ge" => commands::ge(interp),
                "gt" => commands::gt(interp),
                "le" => commands::le(interp),
                "lt" => commands::lt(interp),
                "and" => commands::and(interp),
                "or" => commands::or(interp),
                "not" => commands::not(interp),
                "true" => commands::ps_true(interp),
                "false" => commands::ps_false(interp),
                "if" => commands::ps_if(interp),
                "ifelse" => commands::ps_ifelse(interp),
                "repeat" => commands::ps_repeat(interp),
                "for" => commands::ps_for(interp),
                "get" => commands::get(interp),
                "getinterval" => commands::getinterval(interp),
                "putinterval" => commands::putinterval(interp),
                "print" => commands::print(interp),
                "=" => commands::eq_print(interp),
                "==" => commands::eq_eq_print(interp),
                "quit" => commands::quit(),
                "help" => commands::ps_help(),
                "-s" => commands::print_full_stack(interp),
                "-l" => commands::set_lexical_scope(interp),
                "-d" => commands::set_dynamic_scope(interp),
                "-m" => commands::print_scope_mode(interp),
                _ => {
                    if let Some(val) = interp.lookup(&name) {
                        match val {
                            PSValue::Proc { body: _, env: _ } => {
                                exec_proc(interp, val);
                            }
                            _ => interp.op_stack.push(val),
                        }
                    } else {
                        println!("Error: unknown command '{}'", name);
                    }
                }
            },
            _ => {}
        }
    }
}
