use std::collections::VecDeque;
fn main() {
    let mut queue = VecDeque::new();
    let mut stack = Stack(vec![1, 2, 3, 4, 5]);

    println!("before: {:?}", stack);
    while let Some(e) = stack.pop() {
        queue.push_back(e);
    }

    while let Some(e) = queue.pop_front() {
        stack.push(e);
    }

    println!("after: {:?}", stack);
}

#[derive(Debug)]
struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    fn push(&mut self, x: T) {
        self.0.push(x);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}
