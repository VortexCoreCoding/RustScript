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
        env: Option<Vec<HashMap<String, PSValue>>>,
    },

    Dict(HashMap<String, PSValue>),
}