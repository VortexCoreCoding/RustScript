use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum PSValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    Str(String),
    Name(String),
    Literal(String),

    Proc {
        body: Vec<PSValue>,
        env: Option<Vec<(HashMap<String, PSValue>, i32)>>, // Captured environment for lexical scope
    },

    Dict(HashMap<String, PSValue>, i32), // Dictionary with maxsize (ignored in this implementation)
}
