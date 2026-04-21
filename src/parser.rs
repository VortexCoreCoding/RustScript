use crate::{interpreter, types::PSValue};

pub fn parse(input: &str, interp: &mut interpreter::Interpreter) -> Vec<PSValue> {
    let tokens: Vec<String> = tokenize(input);
    let mut pos = 0;
    parse_tokens(&tokens, &mut pos, interp)
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' => tokens.push("{".to_string()),
            '}' => tokens.push("}".to_string()),
            '(' => {
                let mut string_content: String = String::new();
                while let Some(c) = chars.next() {
                    if c == ')' {
                        break;
                    } else if c == '\\' {
                        if let Some(next) = chars.next() {
                            match next {
                                'n' => string_content.push('\n'),
                                't' => string_content.push('\t'),
                                'r' => string_content.push('\r'),
                                '\\' => string_content.push('\\'),
                                '(' => string_content.push('('),
                                ')' => string_content.push(')'),
                                _ => string_content.push(next),
                            }
                        }
                    } else {
                        string_content.push(c);
                    }
                }
                tokens.push(format!("({})", string_content));
            }
            '-' => {
                // Flag to convert from lexical to dynamic and back (-l for lexical, -d for dynamic)
                if let Some(next) = chars.next() {
                    if next == 'l' {
                        tokens.push("-l".to_string()); // TODO: make into a real flag token instead of string
                    } else if next == 'd' {
                        tokens.push("-d".to_string()); // Also it currently triggers on -d in the middle of a word, which is not ideal
                    } else {
                        // If it's not a flag, treat it as part of a number or word
                        let mut word = String::new();
                        word.push('-');
                        word.push(next);
                        while let Some(&next) = chars.peek() {
                            if next.is_whitespace() || next == '{' || next == '}' || next == '(' || next == ')' {
                                break;
                            }
                            word.push(chars.next().unwrap());
                        }
                        tokens.push(word);
                    }
                }
            }
            c if c.is_whitespace() => continue,
            _ => {
                let mut word = String::new();
                word.push(ch);
                while let Some(&next) = chars.peek() {
                    if next.is_whitespace() || next == '{' || next == '}' || next == '(' || next == ')' {
                        break;
                    }
                    word.push(chars.next().unwrap());
                }
                tokens.push(word);
            }
        }
    }

    tokens // returns
}

fn parse_tokens(tokens: &[String], pos: &mut usize, interp: &mut interpreter::Interpreter) -> Vec<PSValue> {
    let mut result = Vec::new();

    while *pos < tokens.len() {
        let tok = &tokens[*pos];
        *pos += 1;

        match tok.as_str() {
            "{" => {
                let proc = parse_tokens(tokens, pos, interp);
                result.push(PSValue::Proc {
                    body: proc,
                    env: Some(interp.dict_stack.snapshot()),
                });
            }
            "}" => {
                return result;
            }
            "-l" => {
                // Flag to convert from lexical to dynamic and back
                interp.set_scope(interpreter::ScopeMode::Lexical);
                println!("Switched to Lexical Scope");
            }
            "-d" => {
                // Flag to convert from lexical to dynamic and back
                interp.set_scope(interpreter::ScopeMode::Dynamic);
                println!("Switched to Dynamic Scope");
            }
            _ => result.push(parse_atom(tok)),
        }
    }

    result
}

fn parse_atom(tok: &str) -> PSValue {
    if tok.contains('.') {
        if let Ok(f) = tok.parse::<f64>() {
            return PSValue::Float(f);
        }
    }

    if let Ok(n) = tok.parse::<i32>() {
        return PSValue::Int(n);
    }

    if tok == "true" {
        return PSValue::Bool(true);
    } else if tok == "false" {
        return PSValue::Bool(false);
    } else if tok.starts_with('(') && tok.ends_with(')') {
        return PSValue::Str(tok[1..tok.len() - 1].to_string());
    } else if tok.starts_with('/') {
        return PSValue::Name(tok[1..].to_string());
    } else {
        return PSValue::Literal(tok.to_string());
    }
}