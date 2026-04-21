use crate::{interpreter::{Interpreter, ScopeMode}, types::PSValue};
use std::io::{self, Write};

// Used for simple = print
fn value_text(value: &PSValue) -> String {
    match value {
        PSValue::Int(n) => n.to_string(),
        PSValue::Float(f) => f.to_string(),
        PSValue::Bool(true) => "true".into(),
        PSValue::Bool(false) => "false".into(),
        PSValue::Str(s) => s.clone(),
        PSValue::Name(n) => format!("{}", n),
        PSValue::Literal(n) => n.clone(),
        PSValue::Dict(d, _) => format!("<<{} items>>", d.len()),
        PSValue::Proc { .. } => "{...}".into(),
        // _ => format!("{:?}", value),
    }
}

// Used for == print
fn debug_value_text(value: &PSValue) -> String {
    match value {
        PSValue::Int(n) => n.to_string(),
        PSValue::Float(f) => f.to_string(),
        PSValue::Bool(true) => "true".into(),
        PSValue::Bool(false) => "false".into(),
        PSValue::Str(s) => format!("({})", s.clone()),
        PSValue::Name(n) => format!("/{}", n),
        PSValue::Literal(n) => n.clone(),
        PSValue::Dict(_, _) => "-dict-".into(),
        // Print the body of the proc for debugging, but not the captured environment
        PSValue::Proc { body, env: _ } => {
            let debug_body = body.iter().map(|v| debug_value_text(v)).collect::<Vec<_>>();
            format!("{{{:?}}}", debug_body)
        }
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
    interp.op_stack.push(PSValue::Int(interp.op_stack.len() as i32));
}


// DICTIONARY COMMANDS //

// DONE: Creates a new empty dictionary and pushes it onto the operand stack. int -> dict
pub fn dict(interp: &mut Interpreter) {
    // Get int maxsize argument (ignored in this implementation)
    if interp.op_stack.len() < 1 {
        println!("Error: dict expects 1 operand");
        return;
    }
    if let Some(PSValue::Int(maxsize)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the maxsize operand
        interp.op_stack.push(PSValue::Dict(Default::default(), maxsize));
    } else {
        println!("Error: dict expects an integer operand");
    }
}

// DONE: Returns the maximum number of entries allowed in a dictionary. Stack: dict -> maxsize
pub fn maxlength(interp: &mut Interpreter) {
    if interp.op_stack.len() < 1 {
        println!("Error: maxlength expects 1 operand");
        return;
    }
    if let Some(PSValue::Dict(_, a)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the dict operand
        interp.op_stack.push(PSValue::Int(a as i32));
    } else {
        println!("Error: maxlength expects a dictionary operand");
    }
}

// DONE: Begins a new dictionary context by pushing a dictionary onto the dictionary stack. Stack: dict -> (dict on dict stack)
pub fn begin(interp: &mut Interpreter) {
    if let Some(PSValue::Dict(d , a)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the dict operand
        interp.dict_stack.begin((d, a)); // Push the dict with maxsize (ignored in this implementation)
    }
    else {
        println!("Error: begin expects a dictionary operand");
    }
}

// DONE: Ends the current dictionary context by popping the top dictionary from the dictionary stack. Stack: -> (removes top dict from dict stack)
pub fn end(interp: &mut Interpreter) {
    interp.dict_stack.end();
}

// DONE: Defines a name in the current dictionary. Stack: val /name -> (defines name as val)
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

// DONE: Returns the length of a string or dict. Stack: string -> length
pub fn length(interp: &mut Interpreter) {
    if interp.op_stack.len() < 1 {
        println!("Error: length expects 1 operand");
        return;
    }
    if let Some(PSValue::Str(s)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the string operand
        interp.op_stack.push(PSValue::Int(s.len() as i32));
    }
    else if let Some(PSValue::Dict(d, _)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the dict operand
        interp.op_stack.push(PSValue::Int(d.len() as i32));
    }
    else {
        println!("Error: length expects a string or dictionary operand");
    }
}

// DONE: Returns the ASCII value of a character at a given index in a string. Stack: string index -> char_code
pub fn get(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: get expects 2 operands");
        return;
    }
    if let (Some(PSValue::Str(s)), Some(PSValue::Int(index))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        if index >= 0 && (index as usize) < s.len() {
            interp.op_stack.pop(); // Remove the index operand
            interp.op_stack.pop(); // Remove the string operand
            let byte = s.as_bytes()[index as usize] as i32;
            interp.op_stack.push(PSValue::Int(byte));
        }
        else {
            println!("Error: get index out of bounds");
        }
    }
    else {
        println!("Error: get expects a string and an integer operand");
    }
}

// DONE: Extracts a substring from a string. Stack: string index count -> substring
pub fn getinterval(interp: &mut Interpreter) {
    if interp.op_stack.len() < 3 {
        println!("Error: getinterval expects 3 operands");
        return;
    }
    if let (Some(PSValue::Int(count)), Some(PSValue::Int(index)), Some(PSValue::Str(s))) =
        (interp.op_stack.peek_n(0).cloned(), interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(2).cloned())
    {
        let start = index as usize;
        let len = count as usize;
        if start <= s.len() && start + len <= s.len() {
            interp.op_stack.pop(); // Remove count operand
            interp.op_stack.pop(); // Remove index operand
            interp.op_stack.pop(); // Remove string operand
            let substr: String = s[start..start + len].to_string();
            interp.op_stack.push(PSValue::Str(substr));
        }
        else {
            println!("Error: getinterval index/count out of bounds");
        }
    }
    else {
        println!("Error: getinterval expects a string and two integer operands");
    }
}

// DONE: Copies a source string into a destination string starting at a given index. Stack: dest index source -> (modifies dest)
pub fn putinterval(interp: &mut Interpreter) {
    if interp.op_stack.len() < 3 {
        println!("Error: putinterval expects 3 operands");
        return;
    }

    if let (Some(PSValue::Str(source)), Some(PSValue::Int(index)), Some(PSValue::Str(mut dest))) =
        (interp.op_stack.peek_n(0).cloned(), interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(2).cloned())
    {
        let start = index as usize;
        if start + source.len() <= dest.len() {
            interp.op_stack.pop(); // Remove source operand
            interp.op_stack.pop(); // Remove index operand
            interp.op_stack.pop(); // Remove dest operand
            dest.replace_range(start..start + source.len(), &source);
        }
        else {
            println!("Error: putinterval index/source length out of bounds");
        }
        interp.op_stack.push(PSValue::Str(dest));
    }
    else {
        println!("Error: putinterval expects two strings and an integer operand");
    }
}


// CONTROL FLOW COMMANDS //

// DONE: Executes a procedure if the condition is true. Stack: bool proc -> (executes proc if bool is true)
pub fn ps_if(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: if expects 2 operands");
        return;
    }

    if let (Some(PSValue::Bool(c)), Some(PSValue::Proc { body, env })) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        let proc_val = PSValue::Proc { body, env };
        interp.op_stack.pop();
        interp.op_stack.pop();
        if c {
            crate::exec_proc(interp, proc_val);
        }
    }
    else {
        println!("Error: if expects (bool proc)");
    }
}

// DONE: Executes one of two procedures based on a condition. Stack: bool proc_true proc_false -> (executes proc_true if bool, else proc_false)
pub fn ps_ifelse(interp: &mut Interpreter) {
    if interp.op_stack.len() < 3 {
        println!("Error: ifelse expects 3 operands");
        return;
    }
    if let (
        Some(proc_false @ PSValue::Proc { .. }),
        Some(proc_true @ PSValue::Proc { .. }),
        Some(PSValue::Bool(cond)),
    ) = (
        interp.op_stack.peek_n(0).cloned(),
        interp.op_stack.peek_n(1).cloned(),
        interp.op_stack.peek_n(2).cloned(),
    ) {
        interp.op_stack.pop(); // Remove proc_false operand
        interp.op_stack.pop(); // Remove proc_true operand
        interp.op_stack.pop(); // Remove condition operand
        if cond {
            crate::exec_proc(interp, proc_true);
        } else {
            crate::exec_proc(interp, proc_false);
        }
    }
    else {
        println!("Error: ifelse expects (bool proc_true proc_false)");
    }
}

// DONE: Repeats a procedure a specified number of times. Stack: n proc -> (executes proc n times)
pub fn ps_repeat(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: repeat expects 2 operands");
        return;
    }
    if let (Some(proc @ PSValue::Proc { .. }), Some(PSValue::Int(n))) =
        (interp.op_stack.peek_n(0).cloned(), interp.op_stack.peek_n(1).cloned())
    {
        interp.op_stack.pop(); // Remove proc operand
        interp.op_stack.pop(); // Remove n operand
        for _ in 0..n {
            crate::exec_proc(interp, proc.clone());
        }
    }
    else {
        println!("Error: repeat expects (int proc)");
    }
}

// DONE: Executes a procedure for a range of values. Stack: start step end proc -> (pushes i and executes proc for i from start to end by step)
pub fn ps_for(interp: &mut Interpreter) {
    if interp.op_stack.len() < 4 {
        println!("Error: for expects 4 operands");
        return;
    }
    if let (
        Some(proc @ PSValue::Proc { .. }),
        Some(PSValue::Int(end)),
        Some(PSValue::Int(step)),
        Some(PSValue::Int(start)),
    ) = (
        interp.op_stack.peek_n(0).cloned(),
        interp.op_stack.peek_n(1).cloned(),
        interp.op_stack.peek_n(2).cloned(),
        interp.op_stack.peek_n(3).cloned(),
    ) {
        interp.op_stack.pop(); // Remove proc operand
        interp.op_stack.pop(); // Remove end operand
        interp.op_stack.pop(); // Remove step operand
        interp.op_stack.pop(); // Remove start operand

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
    else {
        println!("Error: for expects (int start int step int end proc)");
    }
}

// DONE: Simply exits the interpreter
pub fn quit() {
    std::process::exit(0);
}


// LOGIC COMMANDS //

// DONE: Equality test. Stack: a b -> (a == b)
pub fn eq(interp: &mut Interpreter) {
    if let (Some(b), Some(a)) = (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned()) {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a == b));
    }
    else {
        println!("Error: eq expects 2 operands");
    }
}

// DONE: Inequality test. Stack: a b -> (a != b)
pub fn ne(interp: &mut Interpreter) {
    if let (Some(b), Some(a)) = (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned()) {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a != b));
    }
    else {
        println!("Error: ne expects 2 operands");
    }
}

// DONE: Greater than or equal test string and int. Stack: a b -> (a >= b)
pub fn ge(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: ge expects 2 operands");
        return;
    }
    if let (Some(PSValue::Int(a)), Some(PSValue::Int(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a >= b));
    }
    else if let (Some(PSValue::Str(a)), Some(PSValue::Str(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a >= b));
    }
    else {
        println!("Error: ge expects two integers or two strings");
    }
}

// DONE: Greater than test. Stack: a b -> (a > b)
pub fn gt(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: gt expects 2 operands");
        return;
    }
    if let (Some(PSValue::Int(a)), Some(PSValue::Int(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a > b));
    }
    else if let (Some(PSValue::Str(a)), Some(PSValue::Str(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a > b));
    }
    else {
        println!("Error: gt expects two integers or two strings");
    }
}

// DONE: Less than or equal test. Stack: a b -> (a <= b)
pub fn le(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: le expects 2 operands");
        return;
    }
    if let (Some(PSValue::Int(a)), Some(PSValue::Int(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a <= b));
    }
    else if let (Some(PSValue::Str(a)), Some(PSValue::Str(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a <= b));
    }
    else {
        println!("Error: le expects two integers or two strings");
    }
}

// DONE: Less than test. Stack: a b -> (a < b)
pub fn lt(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: lt expects 2 operands");
        return;
    }
    if let (Some(PSValue::Int(a)), Some(PSValue::Int(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a < b));
    }
    else if let (Some(PSValue::Str(a)), Some(PSValue::Str(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a < b));
    }
    else {
        println!("Error: lt expects two integers or two strings");
    }
}

// DONE: Logical AND. Stack: a b -> (a && b)
pub fn and(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: and expects 2 operands");
        return;
    }
    if let (Some(PSValue::Bool(b)), Some(PSValue::Bool(a))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a && b));
    }
    else if let (Some(PSValue::Int(a)), Some(PSValue::Int(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        let a_bool = a != 0;
        let b_bool = b != 0;
        if a_bool && b_bool {
            interp.op_stack.push(PSValue::Int(1));
        } else {
            interp.op_stack.push(PSValue::Int(0));
        }
    }
    else {
        println!("Error: and expects 2 boolean or integer operands");
    }
}

// DONE: Logical OR. Stack: a b -> (a || b)
pub fn or(interp: &mut Interpreter) {
    if interp.op_stack.len() < 2 {
        println!("Error: or expects 2 operands");
        return;
    }
    if let (Some(PSValue::Bool(b)), Some(PSValue::Bool(a))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        interp.op_stack.push(PSValue::Bool(a || b));
    }
    else if let (Some(PSValue::Int(a)), Some(PSValue::Int(b))) =
        (interp.op_stack.peek_n(1).cloned(), interp.op_stack.peek_n(0).cloned())
    {
        interp.op_stack.pop();
        interp.op_stack.pop();
        let a_bool = a != 0;
        let b_bool = b != 0;
        if a_bool || b_bool {
            interp.op_stack.push(PSValue::Int(1));
        } else {
            interp.op_stack.push(PSValue::Int(0));
        }
    }
    else {
        println!("Error: or expects 2 boolean or integer operands");
    }
}

// DONE: Logical NOT. Stack: bool -> !bool
pub fn not(interp: &mut Interpreter) {
    if let Some(PSValue::Bool(value)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the operand
        interp.op_stack.push(PSValue::Bool(!value));
    }
    else if let Some(PSValue::Int(value)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the operand
        let a = value == 0;
        interp.op_stack.push(PSValue::Int(if a { 1 } else { 0 }));
    }
    else {
        println!("Error: not expects a boolean or integer operand");
    }
}

// DONE: Pushes the boolean value true onto the stack. Stack: -> true
pub fn ps_true(interp: &mut Interpreter) {
    interp.op_stack.push(PSValue::Bool(true));
}

// DONE: Pushes the boolean value false onto the stack. Stack: -> false
pub fn ps_false(interp: &mut Interpreter) {
    interp.op_stack.push(PSValue::Bool(false));
}


// INPUT AND OUTPUT COMMANDS //

// DONE: Prints the top string on the stack. Stack: string -> (prints string)
pub fn print(interp: &mut Interpreter) {
    if let Some(PSValue::Str(value)) = interp.op_stack.peek().cloned() {
        interp.op_stack.pop(); // Remove the string operand
        println!("{}", value);
        io::stdout().flush().ok();
    }
}

// DONE: Prints the top value on the stack using its text representation. Stack: val -> (prints val as text)
pub fn eq_print(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        println!("{}", value_text(&value));
    }
}

// DONE: Prints the top value on the stack using its debug representation. eg (string) using parenthesis
pub fn eq_eq_print(interp: &mut Interpreter) {
    if let Some(value) = interp.op_stack.pop() {
        println!("{}", debug_value_text(&value));
    }
}

// DONE: Prints a help message listing available commands
pub fn ps_help () {
    println!("Available commands:");
    println!("Arithmetic: add, sub, mul, div, idiv, mod, abs, neg, ceiling, floor, round, sqrt");
    println!("Stack Manipulation: dup, exch, pop, copy, clear, count");
    println!("Dictionary: dict, maxlength, begin, end, def");
    println!("String: length, get, getinterval, putinterval");
    println!("Control Flow: if, ifelse, repeat, for");
    println!("Logic: eq, ne, ge, gt, le, lt, and, or, not");
    println!("Input/Output: print, eq_print (prints text), eq_eq_print (prints debug)");
    println!("Other/Debugging: quit, -l (lexical scope), -d (dynamic scope), -m (see current mode), -s (full stack print), help");
}

// DONE: Prints the full contents of the operand stack with indices. Stack: -> (prints full stack)
pub fn print_full_stack(interp: &mut Interpreter) {
    println!("Full Stack:");
    for i in 0..interp.op_stack.len() {
        if let Some(value) = interp.op_stack.peek_n(i).cloned() {
            println!("{}: {}", i, debug_value_text(&value));
        }
    }
}

pub fn set_lexical_scope(interp: &mut Interpreter) {
    interp.set_scope(ScopeMode::Lexical);
}

pub fn set_dynamic_scope(interp: &mut Interpreter) {
    interp.set_scope(ScopeMode::Dynamic);
}

pub fn print_scope_mode(interp: &mut Interpreter) {
    match interp.get_scope() {
        ScopeMode::Lexical => println!("Current scope mode: Lexical"),
        ScopeMode::Dynamic => println!("Current scope mode: Dynamic"),
    }
}