use crate::{dict::DictStack, stack::OperandStack, types::PSValue};

pub enum ScopeMode {
    Dynamic,
    Lexical,
}

pub struct Interpreter {
    pub op_stack: OperandStack,
    pub dict_stack: DictStack,
    pub scope: ScopeMode,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            op_stack: OperandStack::new(),
            dict_stack: DictStack::new(), // Initialize with a global dictionary
            scope: ScopeMode::Dynamic,
        }
    }

    pub fn set_scope(&mut self, mode: ScopeMode) {
        self.scope = mode;
    }

    pub fn get_scope(&mut self) -> &ScopeMode {
        &self.scope
    }

    pub fn lookup(&self, name: &str) -> Option<PSValue> {
        // For dynamic scope, we search the dict stack from top to bottom
        for dict in self.dict_stack.iter().rev() {
            if let Some(val) = dict.0.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    pub fn lookup_in_env(
        env: &Vec<std::collections::HashMap<String, PSValue>>,
        key: &str,
    ) -> Option<PSValue> {
        for dict in env.iter().rev() {
            if let Some(v) = dict.get(key) {
                return Some(v.clone());
            }
        }
        None
    }
}
