#![allow(non_snake_case)]
use std::mem;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        return List { head: None };
    }

    pub fn push(&mut self, item: T) {
        let nextNode = Box::new(Node {
            value: item,
            next: self.head.take(),
        });

        self.head = Some(nextNode);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.head.take().map(|node| {
            self.head = node.next;
            node.value
        });
    }

    pub fn peek(&self) -> Option<&T> {
        return self.head.as_ref().map(|node| &node.value);
    }
}