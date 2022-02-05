#![allow(non_snake_case)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn length(&self) -> usize {
        return self.stack.len();
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.stack.pop();
    }

    pub fn push(&mut self, item: T) {
        return self.stack.push(item);
    }

    pub fn isEmpty(&self) -> bool {
        return self.stack.is_empty();
    }

    pub fn peek(&self) -> Option<&T> {
        return self.stack.last();
    }
}