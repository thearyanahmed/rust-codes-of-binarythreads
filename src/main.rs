#![allow(non_snake_case)]

mod ds;

fn main() {
    let mut q : ds::queue::Queue<isize> = ds::queue::Queue::new();

    q.enqueue(1);
    let item = q.dequeue();
    assert_eq!(item, 1);
    assert_eq!(q.isEmpty(), true);
    q.peek();
    println!("length {}",q.length());
}

