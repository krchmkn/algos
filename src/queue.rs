/// Queue

struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert!(!queue.is_empty());
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::new();
        assert_eq!(queue.dequeue(), None);
        queue.enqueue(1);
        assert_eq!(queue.dequeue(), Some(1));
    }

    #[test]
    fn is_empty() {
        let queue = Queue::<i32>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn len() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
    }
}
