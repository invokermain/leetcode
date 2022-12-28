use std::collections::VecDeque;

struct MinStack {
    stack: VecDeque<i32>,
    min_index_stack: VecDeque<usize>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            min_index_stack: VecDeque::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push_back(val);
            self.min_index_stack.push_back(0);
        } else {
            let current_minimum = self.stack[*self.min_index_stack.back().unwrap()];

            if val < current_minimum {
                self.min_index_stack.push_back(self.stack.len());
            }

            self.stack.push_back(val);
        }
    }

    fn pop(&mut self) {
        if *self.min_index_stack.back().unwrap() == self.stack.len() - 1 {
            self.min_index_stack.pop_back();
        }

        self.stack.pop_back();
    }

    fn top(&self) -> i32 {
        *self.stack.back().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.stack[*self.min_index_stack.back().unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test_case_1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);

        assert_eq!(min_stack.get_min(), -2)
    }

    #[test]
    fn test_case_2() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);

        assert_eq!(min_stack.get_min(), -2)
    }

    #[test]
    fn test_case_3() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);

        assert_eq!(min_stack.get_min(), -3)
    }

    #[test]
    fn test_case_4() {
        let mut min_stack = MinStack::new();
        min_stack.push(-1);
        min_stack.push(-2);
        min_stack.pop();

        assert_eq!(min_stack.get_min(), -1)
    }
}
