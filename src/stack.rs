use crate::types::PSValue;

#[derive(Debug)]
pub struct OperandStack {
    stack: Vec<PSValue>,
}

impl OperandStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, val: PSValue) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Option<PSValue> {
        self.stack.pop()
    }

    pub fn pop_n(&mut self, n: usize) -> Option<PSValue> {
        for _ in 0..n {
            if self.stack.pop().is_none() {
                return None; // Not enough elements to pop
            }
        }
        self.stack.pop()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> Option<&PSValue> {
        self.stack.last()
    }

    pub fn peek_n(&self, n: usize) -> Option<&PSValue> {
        let index = self.stack.len().checked_sub(n + 1)?;
        self.stack.get(index).clone()
    }

    pub fn dup(&mut self) {
        if let Some(top) = self.stack.last().cloned() {
            self.stack.push(top);
        }
    }

    pub fn exch(&mut self) {
        let len = self.stack.len();
        if len >= 2 {
            self.stack.swap(len - 1, len - 2);
        }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn print_stack(&self) {
        for val in &self.stack {
            println!("{:?}", val);
        }
    }
}
