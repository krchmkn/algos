/// Heap
/// mini heap
struct Heap {
    heap: Vec<i32>,
}

impl Heap {
    fn new() -> Self {
        Heap { heap: Vec::new() }
    }

    fn insert(&mut self, value: i32) {
        self.heap.push(value);
        self.up(self.heap.len() - 1);
    }

    fn extract_min(&mut self) -> Option<i32> {
        if self.heap.is_empty() {
            return None;
        }

        let min = self.heap[0];
        let last = self.heap.pop().unwrap();

        if !self.heap.is_empty() {
            self.heap[0] = last;
            self.down(0);
        }

        Some(min)
    }

    fn up(&mut self, index: usize) {
        let mut current = index;

        while current > 0 {
            let parent = (current - 1) / 2;
            if self.heap[current] < self.heap[parent] {
                self.heap.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }

    fn down(&mut self, index: usize) {
        let mut smallest = index;
        let left = 2 * index + 1;
        let right = 2 * index + 2;

        if left < self.heap.len() && self.heap[left] < self.heap[smallest] {
            smallest = left;
        }

        if right < self.heap.len() && self.heap[right] < self.heap[smallest] {
            smallest = right;
        }

        if smallest != index {
            self.heap.swap(index, smallest);
            self.down(smallest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_extract_min() {
        let mut heap = Heap::new();
        heap.insert(4);
        heap.insert(2);
        heap.insert(8);

        assert_eq!(heap.extract_min(), Some(2));
    }
}
