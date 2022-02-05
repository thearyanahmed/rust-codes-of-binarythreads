#![allow(non_snake_case)]

pub struct Queue<T> {
	queue: Vec<T>,
}

impl <T> Queue<T> {
	pub fn new() -> Self {
		return Queue{ queue: Vec::new() };
	}

	pub fn enqueue(&mut self, item: T) {
		self.queue.push(item);
	}

	pub fn dequeue(&mut self) -> T {
		return self.queue.remove(0);
	}

	pub fn length(&self) -> usize {
		return self.queue.len();
	}

	pub fn isEmpty(&self) -> bool {
		return self.queue.is_empty();
	}

	pub fn peek(&self) -> Option<&T> {
	    return self.queue.first();
	}	
}