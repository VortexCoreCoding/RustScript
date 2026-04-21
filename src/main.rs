use ps_interpreter::{parse, Interpreter};
use std::io::{self, Write};

fn repl() {
    let mut interp = Interpreter::new();
    let stdin = io::stdin();

    loop {
        // prompt: ps<x>
        let stack_size = interp.op_stack.count();
        print!("ps<{}> ", stack_size);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        let tokens = parse(trimmed, &mut interp);
        println!("TOKENS: {:?}", tokens);
        ps_interpreter::execute(&mut interp, tokens);

        // Optional: show stack after each command (great for debugging / viva)
        interp.op_stack.print_stack();
        // You might want to add a getter instead of exposing internals
        // quick hack:
        // println!("{:?}", interp.op_stack);
    }
}

fn main() {
    repl();
}