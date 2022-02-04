#![allow(non_snake_case)]

use std::collections::HashMap;

const FIB_BASE : u64 = 1;

pub fn recursiveFibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return FIB_BASE;
    }

    return recursiveFibonacci(n - 1) + recursiveFibonacci(n - 2);
}

pub fn optimisedFibonacci(n: u64, map: &mut HashMap<u64,u64>) -> u64 {
    match n {
        0 | 1 => FIB_BASE,
        n => {
            // if map has a value for key n, return it
            if map.contains_key(&n) {
                return *map.get(&n).unwrap();
            }

            // else figure our the fibonacci value.
            let v = optimisedFibonacci(n - 1, map) + optimisedFibonacci(n - 2, map);

            // update the map
            map.insert(n,v);

            // return the new fibonacci val
            return v;
        }
    }
}
