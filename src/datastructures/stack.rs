#![allow(dead_code)]

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}
