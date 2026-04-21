use crate::types::PSValue;
use std::collections::HashMap;

pub struct DictStack {
    stack: Vec<(HashMap<String, PSValue>, i32)>, // Each dictionary is paired with its maxsize (ignored in this implementation)
}

impl DictStack {
    pub fn new() -> Self {
        Self {
            stack: vec![(HashMap::new(), i32::MAX)], // global scope
        }
    }

    pub fn snapshot(&self) -> Vec<(HashMap<String, PSValue>, i32)> {
        self.stack.clone()
    }

    pub fn replace(&mut self, new_stack: Vec<(HashMap<String, PSValue>, i32)>) {
        self.stack = new_stack;
    }

    pub fn define(&mut self, key: String, val: PSValue) {
        if let Some(dict) = self.stack.last_mut() {
            dict.0.insert(key, val);
        }
    }

    pub fn lookup_dynamic(&self, key: &str) -> Option<PSValue> {
        for dict in self.stack.iter().rev() {
            if let Some(v) = dict.0.get(key) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (HashMap<String, PSValue>, i32)> {
        self.stack.iter()
    }

    pub fn begin(&mut self, dict: (HashMap<String, PSValue>, i32)) {
        self.stack.push(dict);
    }

    pub fn end(&mut self) {
        if self.stack.len() > 1 {
        self.stack.pop();
        } else {
            println!("Error: Cannot end global dictionary scope");
        }
    }
}