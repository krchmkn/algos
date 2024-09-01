/// Stack

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
    }
}
