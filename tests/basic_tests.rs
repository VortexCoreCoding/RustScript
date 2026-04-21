// filepath: PostScriptInterpreter\tests\basic_tests.rs
use ps_interpreter::interpreter::{Interpreter, ScopeMode};
use ps_interpreter::parser::parse;
use ps_interpreter::types::PSValue;
use ps_interpreter::*;

// helper to run a program and return top of stack
fn run(program: &str, scope: ScopeMode) -> Option<PSValue> {
    let mut interp = Interpreter::new();
    interp.set_scope(scope);

    let tokens = parse(program, &mut interp);
    execute(&mut interp, tokens);

    interp.op_stack.pop()
}

#[test]
fn test_add() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(3));
    interp.op_stack.push(PSValue::Int(4));

    add(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(7)));
}

#[test]
fn test_sub() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(10));
    interp.op_stack.push(PSValue::Int(3));

    sub(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(7)));
}

#[test]
fn test_mul() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(6));
    interp.op_stack.push(PSValue::Int(7));

    mul(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(42)));
}

#[test]
fn test_div() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(8));
    interp.op_stack.push(PSValue::Int(2));

    div(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Float(4.0)));
}

#[test]
fn test_idiv() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(9));
    interp.op_stack.push(PSValue::Int(2));

    idiv(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(4)));
}

#[test]
fn test_mod() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(9));
    interp.op_stack.push(PSValue::Int(4));

    ps_mod(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
}

#[test]
fn test_abs_neg_ceiling_floor_round_sqrt() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Int(-5));
    abs(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(5)));

    interp.op_stack.push(PSValue::Int(5));
    neg(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(-5)));

    interp.op_stack.push(PSValue::Float(2.3));
    ceiling(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(3)));

    interp.op_stack.push(PSValue::Float(2.7));
    floor(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(2)));

    interp.op_stack.push(PSValue::Float(2.6));
    round(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(3)));

    interp.op_stack.push(PSValue::Int(16));
    sqrt(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Float(4.0)));
}

#[test]
fn test_dup() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(1));

    dup(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
}

#[test]
fn test_exch() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));

    exch(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(2)));
}

#[test]
fn test_ps_def() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Name("x".into()));
    interp.op_stack.push(PSValue::Int(99));

    ps_def(&mut interp);

    assert_eq!(interp.lookup("x"), Some(PSValue::Int(99)));
}

#[test]
fn test_ps_if() {
    let mut interp = Interpreter::new();
    let proc = PSValue::Proc {
        body: vec![PSValue::Int(42)],
        env: None,
    };

    interp.op_stack.push(PSValue::Bool(true));
    interp.op_stack.push(proc);

    ps_if(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(42)));
}

#[test]
fn test_ps_ifelse() {
    let mut interp = Interpreter::new();
    let true_proc = PSValue::Proc {
        body: vec![PSValue::Int(1)],
        env: None,
    };
    let false_proc = PSValue::Proc {
        body: vec![PSValue::Int(0)],
        env: None,
    };

    interp.op_stack.push(PSValue::Bool(false));
    interp.op_stack.push(true_proc);
    interp.op_stack.push(false_proc);

    ps_ifelse(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(0)));
}

#[test]
fn test_ps_repeat() {
    let mut interp = Interpreter::new();
    let proc = PSValue::Proc {
        body: vec![PSValue::Int(7)],
        env: None,
    };

    interp.op_stack.push(PSValue::Int(3));
    interp.op_stack.push(proc);

    ps_repeat(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(7)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(7)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(7)));
}

#[test]
fn test_ps_for() {
    let mut interp = Interpreter::new();
    let proc = PSValue::Proc {
        body: vec![PSValue::Int(1)],
        env: None,
    };

    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(3));
    interp.op_stack.push(proc);

    ps_for(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(3)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(2)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
}

#[test]
fn test_length() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Str("hello".into()));
    length(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(5)));
}

#[test]
fn test_get() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Str("abc".into()));
    interp.op_stack.push(PSValue::Int(1));
    get(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int('b' as i32)));
}

#[test]
fn test_getinterval() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Str("hello".into()));
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(3));
    getinterval(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Str("ell".into())));
}

#[test]
fn test_putinterval() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Str("hello".into()));
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Str("oo".into()));
    putinterval(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Str("hoolo".into())));
}

#[test]
fn test_pop() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));

    pop(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
    assert_eq!(interp.op_stack.pop(), None);
}

#[test]
fn test_copy() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));
    interp.op_stack.push(PSValue::Int(3));
    interp.op_stack.push(PSValue::Int(2));

    copy(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(3)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(2)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(3)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(2)));
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(1)));
}

#[test]
fn test_clear() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));

    clear(&mut interp);

    assert_eq!(interp.op_stack.pop(), None);
}

#[test]
fn test_count() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));

    count(&mut interp);

    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(2)));
}

#[test]
fn test_dict() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Int(10));
    dict(&mut interp);

    if let Some(PSValue::Dict(_, 10)) = interp.op_stack.pop() {
        // Dict created successfully
    } else {
        panic!("Dict not created");
    }
}

#[test]
fn test_maxlength() {
    let mut interp = Interpreter::new();
    interp.op_stack.push(PSValue::Int(5));
    dict(&mut interp);
    maxlength(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Int(5)));
}

#[test]
fn test_eq_ne() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(1));
    eq(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));

    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));
    ne(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));
}

#[test]
fn test_ge_gt_le_lt() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Int(2));
    interp.op_stack.push(PSValue::Int(1));
    ge(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));

    interp.op_stack.push(PSValue::Int(2));
    interp.op_stack.push(PSValue::Int(1));
    gt(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));

    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));
    le(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));

    interp.op_stack.push(PSValue::Int(1));
    interp.op_stack.push(PSValue::Int(2));
    lt(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));
}

#[test]
fn test_and_or_not() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Bool(true));
    interp.op_stack.push(PSValue::Bool(false));
    and(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(false)));

    interp.op_stack.push(PSValue::Bool(true));
    interp.op_stack.push(PSValue::Bool(false));
    or(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));

    interp.op_stack.push(PSValue::Bool(true));
    not(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(false)));
}

#[test]
fn test_true_false() {
    let mut interp = Interpreter::new();

    ps_true(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));

    ps_false(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(false)));
}

#[test]
fn test_lookup_searches_dict_stack() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Name("x".into()));
    interp.op_stack.push(PSValue::Int(10));
    ps_def(&mut interp);

    interp.op_stack.push(PSValue::Int(3));
    dict(&mut interp);
    begin(&mut interp);

    interp.op_stack.push(PSValue::Name("x".into()));
    interp.op_stack.push(PSValue::Int(20));
    ps_def(&mut interp);

    assert_eq!(interp.lookup("x"), Some(PSValue::Int(20)));

    end(&mut interp);

    assert_eq!(interp.lookup("x"), Some(PSValue::Int(10)));
}

#[test]
fn test_dynamic_scoping_basic() {
    let program = "
        /x 10 def
        /f { x } def
        /x 20 def
        f
    ";

    let result = run(program, ScopeMode::Dynamic);

    assert_eq!(result, Some(PSValue::Int(20)));
}

#[test]
fn test_lexical_scoping_basic() {
    let program = "
        /x 10 def
        /f { x } def
        /x 20 def
        f
    ";

    let result = run(program, ScopeMode::Lexical);

    assert_eq!(result, Some(PSValue::Int(10)));
}

#[test]
fn test_dynamic_scoping_nested() {
    let program = "
        /x 10 def
        /g { x } def
        /f { /x 20 def g } def
        f
    ";

    let result = run(program, ScopeMode::Dynamic);

    assert_eq!(result, Some(PSValue::Int(20)));
}

#[test]
fn test_lexical_scoping_nested() {
    let program = "
        /x 10 def
        /g { x } def
        /f { /x 20 def g } def
        f
    ";

    let result = run(program, ScopeMode::Lexical);

    assert_eq!(result, Some(PSValue::Int(10)));
}

#[test]
fn test_closure_behavior() {
    let program = "
        /x 5 def
        /makeAdder {
            /y exch def
            { x y add }
        } def

        /add10 10 makeAdder def
        /x 100 def
        add10
    ";

    let dynamic = run(program, ScopeMode::Dynamic);
    let lexical = run(program, ScopeMode::Lexical);

    // dynamic sees x = 100
    assert_eq!(dynamic, Some(PSValue::Int(110)));

    // lexical captures x = 5
    assert_eq!(lexical, Some(PSValue::Int(15)));
}

// Note: print, =, ==, and quit are not easily testable as they involve output or exit, so skipped.
