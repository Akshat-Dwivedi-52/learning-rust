use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
}
