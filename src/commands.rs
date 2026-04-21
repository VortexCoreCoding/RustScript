use crate::{interpreter::Interpreter, types::PSValue};
use std::io::{self, Write};

fn value_text(value: &PSValue) -> String {
    match value {
        PSValue::Int(n) => n.to_string(),
        PSValue::Float(f) => f.to_string(),
        PSValue::Bool(true) => "true".into(),
        PSValue::Bool(false) => "false".into(),
        PSValue::Str(s) => s.clone(),
        PSValue::Name(n) => format!("/{}", n),
        PSValue::Literal(n) => n.clone(),
        PSValue::Dict(d) => format!("<<{} items>>", d.len()),
        PSValue::Proc { .. } => "{...}".into(),
        // _ => format!("{:?}", value),
    }
}

// fn as_f64(value: PSValue) -> Option<f64> {
//     match value {
//         PSValue::Int(n) => Some(n as f64),
//         PSValue::Float(f) => Some(f),
//         _ => None,
//     }
// }

// fn push_number(interp: &mut Interpreter, value: f64) {
//     if value.fract() == 0.0 {
//         interp.op_stack.push(PSValue::Int(value as i32));
//     } else {
//         interp.op_stack.push(PSValue::Float(value));
//     }
// }


// ARITHMETIC COMMANDS //

// DONE: Adds two integers from the operand stack. Stack: a b -> (a + b)
pub fn add(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: add expects 2 operands");
        return;
    }

    let a = interp.op_stack.peek_n(1).cloned();
    let b = interp.op_stack.peek_n(0).cloned();
    println!("DEBUG: add operands: {:?} and {:?}", a, b);

    match (a, b) {
        (Some(PSValue::Int(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int(a + b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a + b));
        }

        (Some(PSValue::Int(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a as f64 + b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a + b as f64));
        }

        _ => println!("Error: add expects numbers"),
    }
}

// DONE: Subtracts two integers from the operand stack. Stack: a b -> (a - b)
pub fn sub(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: sub expects 2 operands");
        return;
    }

    let a = interp.op_stack.peek_n(1).cloned();
    let b = interp.op_stack.peek_n(0).cloned();

    match (a, b) {
        (Some(PSValue::Int(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int(a - b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a - b));
        }

        (Some(PSValue::Int(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a as f64 - b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a - b as f64));
        }

        _ => println!("Error: sub expects numbers"),
    }
}

// DONE: Multiply two numbers. Stack: a b -> a * b
pub fn mul(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: mul expects 2 operands");
        return;
    }

    let a = interp.op_stack.peek_n(1).cloned();
    let b = interp.op_stack.peek_n(0).cloned();

    match (a, b) {
        (Some(PSValue::Int(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int(a * b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a * b));
        }

        (Some(PSValue::Int(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a as f64 * b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a * b as f64));
        }

        _ => println!("Error: mul expects numbers"),
    }
}

// DONE: Divide two numbers producing a real result. Stack: a b -> a / b
pub fn div(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: div expects 2 operands");
        return;
    }

    let a = interp.op_stack.peek_n(1).cloned();
    let b = interp.op_stack.peek_n(0).cloned();

    match (a, b) {
        (Some(PSValue::Int(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a as f64 / b as f64));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a / b));
        }

        (Some(PSValue::Int(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a as f64 / b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a / b as f64));
        }

        _ => println!("Error: div expects numbers"),
    }
}

// DONE: Integer divide two numbers. Stack: a b -> a idiv b
pub fn idiv(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: idiv expects 2 operands");
        return;
    }

    let a = interp.op_stack.peek_n(1).cloned();
    let b = interp.op_stack.peek_n(0).cloned();

    match (a, b) {
        (Some(PSValue::Int(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int((a / b) as i32));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int((a / b) as i32));
        }

        (Some(PSValue::Int(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int((a as f64 / b) as i32));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int((a / b as f64) as i32));
        }

        _ => println!("Error: idiv expects numbers"),
    }
}

// DONE: Modulo. Stack: a b -> a mod b
pub fn ps_mod(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: mod expects 2 operands");
        return;
    }

    let a = interp.op_stack.peek_n(1).cloned();
    let b = interp.op_stack.peek_n(0).cloned();

    match (a, b) {
        (Some(PSValue::Int(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Int(a % b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a % b));
        }

        (Some(PSValue::Int(a)), Some(PSValue::Float(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a as f64 % b));
        }

        (Some(PSValue::Float(a)), Some(PSValue::Int(b))) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.op_stack.push(PSValue::Float(a % b as f64));
        }

        _ => println!("Error: mod expects numbers"),
    }
}

// DONE: Absolute value. Stack: x -> abs(x)
pub fn abs(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        match value {
            PSValue::Int(n) => interp.op_stack.push(PSValue::Int(n.abs())),
            PSValue::Float(f) => interp.op_stack.push(PSValue::Float(f.abs())),
            _ => {
                println!("Error: abs expects a number");
                interp.op_stack.push(value); // Push the original value back if it's not a number
            }
        }
    }
}

// DONE: Negate a number. Stack: x -> -x
pub fn neg(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        match value {
            PSValue::Int(n) => interp.op_stack.push(PSValue::Int(-n)),
            PSValue::Float(f) => interp.op_stack.push(PSValue::Float(-f)),
            _ => {
                println!("Error: neg expects a number");
                interp.op_stack.push(value); // Push the original value back if it's not a number
            }
        }
    }
}

// DONE: Ceiling of a number. Stack: x -> ceiling(x)
pub fn ceiling(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        match value {
            PSValue::Int(n) => interp.op_stack.push(PSValue::Int(n)),
            PSValue::Float(f) => interp.op_stack.push(PSValue::Int(f.ceil() as i32)),
            _ => {
                println!("Error: ceiling expects a number");
                interp.op_stack.push(value); // Push the original value back if it's not a number
            }
        }
    }
}

// DONE: Floor of a number. Stack: x -> floor(x)
pub fn floor(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        match value {
            PSValue::Int(n) => interp.op_stack.push(PSValue::Int(n)),
            PSValue::Float(f) => interp.op_stack.push(PSValue::Int(f.floor() as i32)),
            _ => {
                println!("Error: floor expects a number");
                interp.op_stack.push(value); // Push the original value back if it's not a number
            }
        }
    }
}

// DONE: Round a number to the nearest integer. Stack: x -> round(x)
pub fn round(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        match value {
            PSValue::Int(n) => interp.op_stack.push(PSValue::Int(n)),
            PSValue::Float(f) => {
                let rounded = if f.is_sign_negative() {
                    (f - 0.5).ceil()
                } else {
                    (f + 0.5).floor()
                };
                interp.op_stack.push(PSValue::Int(rounded as i32));
            }
            _ => {
                println!("Error: round expects a number");
                interp.op_stack.push(value); // Push the original value back if it's not a number
            }
        }
    }
}

// DONE: Square root. Stack: x -> sqrt(x)
pub fn sqrt(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        match value {
            PSValue::Int(n) if n >= 0 => {
                interp.op_stack.push(PSValue::Float((n as f64).sqrt()));
            }
            PSValue::Float(f) if f >= 0.0 => {
                interp.op_stack.push(PSValue::Float(f.sqrt()));
            }
            _ => {
                println!("Error: sqrt expects a non-negative number");
                interp.op_stack.push(value); // Push the original value back if it's not a valid number
            }
        }
    }
}


// STACK MANIPULATION COMMANDS //

// DONE: Duplicates the top element of the operand stack. Stack: a -> a a
pub fn dup(interp: &mut Interpreter) {
    interp.op_stack.dup();
}

// DONE: Exchanges the top two elements of the operand stack. Stack: a b -> b a
pub fn exch(interp: &mut Interpreter) {
    interp.op_stack.exch();
}

// DONE: Pops the top element, without printing
pub fn pop(interp: &mut Interpreter) {
    interp.op_stack.pop();
}

// DONE: Copies the top n elements of the stack
pub fn copy(interp: &mut Interpreter) {
    if interp.op_stack.len() < 1 {
        println!("Error: copy expects at least 1 operand");
        return;
    }

    if let Some(PSValue::Int(n)) = interp.op_stack.peek_n(0).cloned() {
        interp.op_stack.pop(); // Remove the n operand
        if n < 0 {
            println!("Error: copy expects a non-negative integer");
            return;
        }
        if interp.op_stack.len() < n as usize {
            println!("Error: copy expects at least {} operands on the stack", n);
            return;
        }

        let n = n as usize;
        let mut temp = Vec::new();
        
        for _ in 0..n {
            if let Some(value) = interp.op_stack.pop() {
                temp.push(value);
            } else {
                return;
            }
        }

        for value in temp.iter().rev() {
            interp.op_stack.push(value.clone());
        }

        for value in temp.iter().rev() {
            interp.op_stack.push(value.clone());
        }
    }
    else {
        println!("Error: copy expects an integer operand");
    }
}

// DONE: Empties the stack
pub fn clear(interp: &mut Interpreter) {
    interp.op_stack.clear();
}

// DONE: pushes the # of elems in stack to the stack
pub fn count(interp: &mut Interpreter) {
    interp.op_stack.push(PSValue::Int(interp.op_stack.count() as i32));
}


// DICTIONARY COMMANDS //

// Creates a new empty dictionary and pushes it onto the operand stack. Stack: -> dict
pub fn dict(interp: &mut Interpreter) {
    interp.op_stack.push(PSValue::Dict(Default::default()));
}

// Returns the number of entries in a dictionary. Stack: dict -> n
pub fn maxlength(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        match value {
            PSValue::Str(s) => interp.op_stack.push(PSValue::Int(s.len() as i32)),
            PSValue::Dict(d) => interp.op_stack.push(PSValue::Int(d.len() as i32)),
            _ => {}
        }
    }
}

// Begins a new dictionary context by pushing a dictionary onto the dictionary stack. Stack: dict -> (dict on dict stack)
pub fn begin(interp: &mut Interpreter) {
    if let Some(PSValue::Dict(d)) = interp.op_stack.pop() {
        interp.dict_stack.begin(d);
    }
}

// Ends the current dictionary context by popping the top dictionary from the dictionary stack. Stack: -> (removes top dict from dict stack)
pub fn end(interp: &mut Interpreter) {
    interp.dict_stack.end();
}

// Defines a name in the current dictionary. Stack: val /name -> (defines name as val)
pub fn ps_def(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: def expects 2 operands");
        return;
    }

    let key = interp.op_stack.peek_n(1).cloned();
    let val = interp.op_stack.peek_n(0).cloned();

    match key {
        Some(PSValue::Name(name)) => {
            interp.op_stack.pop();
            interp.op_stack.pop();
            interp.dict_stack.define(name, val.unwrap());
        }
        _ => println!("Error: def expects a name"),
    }
}


// STRING COMMANDS //

// Returns the length of a string. Stack: string -> length
pub fn length(interp: &mut Interpreter) {
    if let Some(PSValue::Str(s)) = interp.op_stack.pop() {
        interp.op_stack.push(PSValue::Int(s.len() as i32));
    }
}

// Returns the ASCII value of a character at a given index in a string. Stack: string index -> char_code
pub fn get(interp: &mut Interpreter) {
    if let (Some(PSValue::Int(index)), Some(PSValue::Str(s))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        if index >= 0 && (index as usize) < s.len() {
            let byte = s.as_bytes()[index as usize] as i32;
            interp.op_stack.push(PSValue::Int(byte));
        }
    }
}

// Extracts a substring from a string. Stack: string index count -> substring
pub fn getinterval(interp: &mut Interpreter) {
    if let (Some(PSValue::Int(count)), Some(PSValue::Int(index)), Some(PSValue::Str(s))) =
        (interp.op_stack.pop(), interp.op_stack.pop(), interp.op_stack.pop())
    {
        let start = index as usize;
        let len = count as usize;
        if start <= s.len() && start + len <= s.len() {
            let substr: String = s[start..start + len].to_string();
            interp.op_stack.push(PSValue::Str(substr));
        }
    }
}

// Copies a source string into a destination string starting at a given index. Stack: dest index source -> (modifies dest)
pub fn putinterval(interp: &mut Interpreter) {
    if let (Some(PSValue::Str(source)), Some(PSValue::Int(index)), Some(PSValue::Str(mut dest))) =
        (interp.op_stack.pop(), interp.op_stack.pop(), interp.op_stack.pop())
    {
        let start = index as usize;
        if start + source.len() <= dest.len() {
            dest.replace_range(start..start + source.len(), &source);
        }
        interp.op_stack.push(PSValue::Str(dest));
    }
}


// CONTROL FLOW COMMANDS //

// Executes a procedure if the condition is true. Stack: bool proc -> (executes proc if bool is true)
pub fn ps_if(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: if expects 2 operands");
        return;
    }

    let cond = interp.op_stack.peek_n(1);
    let proc = interp.op_stack.peek_n(0);

    match (cond, proc) {
        (Some(PSValue::Bool(c)), Some(PSValue::Proc { .. })) => {
            let proc_val = interp.op_stack.peek().unwrap().clone();

            if *c {
                crate::exec_proc(interp, proc_val);
            }

            interp.op_stack.pop();
            interp.op_stack.pop();
        }
        _ => {
            println!("Error: if expects (bool proc)");
        }
    }
}

// Executes one of two procedures based on a condition. Stack: bool proc_true proc_false -> (executes proc_true if bool, else proc_false)
pub fn ps_ifelse(interp: &mut Interpreter) {
    if let (
        Some(proc_false @ PSValue::Proc { .. }),
        Some(proc_true @ PSValue::Proc { .. }),
        Some(PSValue::Bool(cond)),
    ) = (
        interp.op_stack.pop(),
        interp.op_stack.pop(),
        interp.op_stack.pop(),
    ) {
        if cond {
            crate::exec_proc(interp, proc_true);
        } else {
            crate::exec_proc(interp, proc_false);
        }
    }
}

// Repeats a procedure a specified number of times. Stack: n proc -> (executes proc n times)
pub fn ps_repeat(interp: &mut Interpreter) {
    if let (Some(proc @ PSValue::Proc { .. }), Some(PSValue::Int(n))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        for _ in 0..n {
            crate::exec_proc(interp, proc.clone());
        }
    }
}

// Executes a procedure for a range of values. Stack: start step end proc -> (pushes i and executes proc for i from start to end by step)
pub fn ps_for(interp: &mut Interpreter) {
    if let (
        Some(proc @ PSValue::Proc { .. }),
        Some(PSValue::Int(end)),
        Some(PSValue::Int(step)),
        Some(PSValue::Int(start)),
    ) = (
        interp.op_stack.pop(),
        interp.op_stack.pop(),
        interp.op_stack.pop(),
        interp.op_stack.pop(),
    ) {
        let mut i = start;

        if step > 0 {
            while i <= end {
                interp.op_stack.push(PSValue::Int(i));
                crate::exec_proc(interp, proc.clone());
                i += step;
            }
        } else {
            while i >= end {
                interp.op_stack.push(PSValue::Int(i));
                crate::exec_proc(interp, proc.clone());
                i += step;
            }
        }
    }
}

// Simply exits the interpreter
pub fn quit() {
    std::process::exit(0);
}


// LOGIC COMMANDS //

pub fn eq(interp: &mut Interpreter) {
    if let (Some(b), Some(a)) = (interp.op_stack.pop(), interp.op_stack.pop()) {
        interp.op_stack.push(PSValue::Bool(a == b));
    }
}

pub fn ne(interp: &mut Interpreter) {
    if let (Some(b), Some(a)) = (interp.op_stack.pop(), interp.op_stack.pop()) {
        interp.op_stack.push(PSValue::Bool(a != b));
    }
}

pub fn ge(interp: &mut Interpreter) {
    if let (Some(PSValue::Int(b)), Some(PSValue::Int(a))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        interp.op_stack.push(PSValue::Bool(a >= b));
    }
}

pub fn gt(interp: &mut Interpreter) {
    if let (Some(PSValue::Int(b)), Some(PSValue::Int(a))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        interp.op_stack.push(PSValue::Bool(a > b));
    }
}

pub fn le(interp: &mut Interpreter) {
    if let (Some(PSValue::Int(b)), Some(PSValue::Int(a))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        interp.op_stack.push(PSValue::Bool(a <= b));
    }
}

pub fn lt(interp: &mut Interpreter) {
    if let (Some(PSValue::Int(b)), Some(PSValue::Int(a))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        interp.op_stack.push(PSValue::Bool(a < b));
    }
}

pub fn and(interp: &mut Interpreter) {
    if let (Some(PSValue::Bool(b)), Some(PSValue::Bool(a))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        interp.op_stack.push(PSValue::Bool(a && b));
    }
}

pub fn or(interp: &mut Interpreter) {
    if let (Some(PSValue::Bool(b)), Some(PSValue::Bool(a))) =
        (interp.op_stack.pop(), interp.op_stack.pop())
    {
        interp.op_stack.push(PSValue::Bool(a || b));
    }
}

pub fn not(interp: &mut Interpreter) {
    if let Some(PSValue::Bool(value)) = interp.op_stack.pop() {
        interp.op_stack.push(PSValue::Bool(!value));
    }
}

pub fn ps_true(interp: &mut Interpreter) {
    interp.op_stack.push(PSValue::Bool(true));
}

pub fn ps_false(interp: &mut Interpreter) {
    interp.op_stack.push(PSValue::Bool(false));
}


// Input and Output Commands //

pub fn print(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        println!("{}", value_text(&value));
        io::stdout().flush().ok();
    }
}

pub fn eq_print(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        println!("{}", value_text(&value));
    }
}

pub fn eq_eq_print(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        println!("{:?}", value);
    }
}