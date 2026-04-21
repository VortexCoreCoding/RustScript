use crate::types::PSValue;
use std::collections::HashMap;

pub struct DictStack {
    stack: Vec<HashMap<String, PSValue>>,
}

impl DictStack {
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()], // global scope
        }
    }

    pub fn snapshot(&self) -> Vec<HashMap<String, PSValue>> {
        self.stack.clone()
    }

    pub fn replace(&mut self, new_stack: Vec<HashMap<String, PSValue>>) {
        self.stack = new_stack;
    }

    pub fn define(&mut self, key: String, val: PSValue) {
        if let Some(dict) = self.stack.last_mut() {
            dict.insert(key, val);
        }
    }

    pub fn lookup_dynamic(&self, key: &str) -> Option<PSValue> {
        for dict in self.stack.iter().rev() {
            if let Some(v) = dict.get(key) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<'_, HashMap<String, PSValue>> {
        self.stack.iter()
    }

    pub fn begin(&mut self, dict: HashMap<String, PSValue>) {
        self.stack.push(dict);
    }

    pub fn end(&mut self) {
        self.stack.pop();
    }
}