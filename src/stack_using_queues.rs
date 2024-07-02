use std::collections::VecDeque;

pub struct MyStack {
    pub stack: VecDeque<i32>,
    // temp: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        let mut temp = VecDeque::new();
        temp.push_back(x);

        while let Some(e) = self.stack.pop_front() {
            temp.push_back(e);
        }

        std::mem::swap(&mut self.stack, &mut temp);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut stack = MyStack::new();

        stack.push(1);
        stack.push(2);

        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert!(!stack.empty());
    }
}
