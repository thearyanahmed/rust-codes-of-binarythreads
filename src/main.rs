use std::collections::HashMap;

mod fibonacci;

fn main() {

    for i in 1..30 {
        let v = fibonacci::recursiveFibonacci(i);

        println!("{} fib {}",i,v);
    }

    let mut map = HashMap::new();

    for i in 1..30 {
        let v = fibonacci::optimisedFibonacci(i, &mut map);

        println!("{} optimised hashmap {}",i, v);
    }
}
