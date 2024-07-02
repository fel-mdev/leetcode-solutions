pub struct CustomStack {
    pub stack: Vec<i32>,
    pub max_size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    pub fn new(max_size: i32) -> Self {
        let capacity = max_size as usize;
        Self {
            stack: Vec::with_capacity(capacity),
            max_size: capacity,
        }
    }

    pub fn push(&mut self, x: i32) {
        if self.max_size > self.stack.len() {
            self.stack.push(x);
        }
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(e) = self.stack.pop() {
            return e;
        } else {
            return -1;
        }
    }

    pub fn increment(&mut self, k: i32, val: i32) {
        if self.stack.len() <= k as usize {
            self.stack.iter_mut().for_each(|e| *e += val);
        } else {
            (0..k).for_each(|i| {
                self.stack[i as usize] += val;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut custom_stack = CustomStack::new(3);
        custom_stack.push(1);
        custom_stack.push(2);

        assert_eq!(custom_stack.pop(), 2);

        custom_stack.push(2);
        custom_stack.push(3);
        custom_stack.push(4);

        custom_stack.increment(5, 100);
        custom_stack.increment(2, 100);

        assert_eq!(custom_stack.pop(), 103);
        assert_eq!(custom_stack.pop(), 202);
        assert_eq!(custom_stack.pop(), 201);
        assert_eq!(custom_stack.pop(), -1);
    }
}
