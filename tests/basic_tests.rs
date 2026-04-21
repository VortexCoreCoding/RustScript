// filepath: PostScriptInterpreter\tests\basic_tests.rs
use ps_interpreter::{
    abs, add, and, begin, ceiling, clear, copy, count, dict, div, dup, end, eq,
    exch, f, floor, ge, get, getinterval, gt, idiv, interpreter::ScopeMode,
    le, length, lt, maxlength, mul, ne, neg, not, or, pop, ps_def, ps_false,
    ps_for, ps_if, ps_ifelse, ps_mod, ps_repeat, ps_true, putinterval, round, sqrt, sub,
    Interpreter, PSValue,
};

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

    dict(&mut interp);

    if let Some(PSValue::Dict(_)) = interp.op_stack.pop() {
        // Dict created successfully
    } else {
        panic!("Dict not created");
    }
}

#[test]
fn test_maxlength() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Str("hello".into()));
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
fn test_true_false_f() {
    let mut interp = Interpreter::new();

    ps_true(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(true)));

    ps_false(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(false)));

    f(&mut interp);
    assert_eq!(interp.op_stack.pop(), Some(PSValue::Bool(false)));
}

#[test]
fn test_lookup_searches_dict_stack() {
    let mut interp = Interpreter::new();

    interp.op_stack.push(PSValue::Name("x".into()));
    interp.op_stack.push(PSValue::Int(10));
    ps_def(&mut interp);

    dict(&mut interp);
    begin(&mut interp);

    interp.op_stack.push(PSValue::Name("x".into()));
    interp.op_stack.push(PSValue::Int(20));
    ps_def(&mut interp);

    assert_eq!(interp.lookup("x"), Some(PSValue::Int(20)));

    end(&mut interp);

    assert_eq!(interp.lookup("x"), Some(PSValue::Int(10)));
}

// Test for both dynamic and lexical scope
#[test]
fn test_scope_modes() {
    let mut interp = Interpreter::new();

    // Define a procedure that looks up 'x'
    let proc = PSValue::Proc {
        body: vec![PSValue::Name("x".into())],
        env: None,
    };
    interp.op_stack.push(proc);
    ps_def(&mut interp); // Define /proc

    // Dynamic scope test
    interp.op_stack.push(PSValue::Name("x".into()));
    interp.op_stack.push(PSValue::Int(10));
    ps_def(&mut interp); // Define /x in global scope

    dict(&mut interp);
    begin(&mut interp);

    interp.op_stack.push(PSValue::Name("x".into()));
    interp.op_stack.push(PSValue::Int(20));
    ps_def(&mut interp); // Define /x in inner scope

    assert_eq!(interp.lookup("proc"), Some(PSValue::Proc {
        body: vec![PSValue::Name("x".into())],
        env: None
    })); // Fails because 
    assert_eq!(interp.lookup("x"), Some(PSValue::Int(20))); // succeeds

    end(&mut interp);

    // Lexical scope test, togglable in the interpreter with flag: 
    let mut interp_lex = Interpreter::new();
    interp_lex.set_scope(ScopeMode::Lexical);

    let proc_lex = PSValue::Proc {
        body: vec![PSValue::Name("x".into())],
        env: None,
    };
    interp_lex.op_stack.push(proc_lex);
    ps_def(&mut interp_lex); // Define /proc

    interp_lex.op_stack.push(PSValue::Name("x".into()));
    interp_lex.op_stack.push(PSValue::Int(10));
    ps_def(&mut interp_lex); // Define /x in global scope

    dict(&mut interp_lex);
    begin(&mut interp_lex);

    interp_lex.op_stack.push(PSValue::Name("x".into()));
    interp_lex.op_stack.push(PSValue::Int(20));
    ps_def(&mut interp_lex); // Define /x in inner scope

    assert_eq!(interp_lex.lookup("proc"), Some(PSValue::Proc {
        body: vec![PSValue::Name("x".into())],
        env: Some(vec![std::collections::HashMap::from([("x".to_string(), PSValue::Int(10))])])
    })); // Fails
}

// Note: print, =, ==, and quit are not easily testable as they involve output or exit, so skipped.