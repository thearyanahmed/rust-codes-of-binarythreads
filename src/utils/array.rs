#![allow(non_snake_case)]
use rand::{self,Rng};

const MAX : usize = 1000000;

pub fn randomInt32Array() -> [i32; MAX] {
    let mut array: [i32; MAX] = [0; MAX];
    let mut ranger = rand::thread_rng();

    for i in 0..MAX {
        array[i] = ranger.gen_range(0..(MAX as i32));
    }

    return array;
}

pub fn randomInt32ArrayMaxSize() -> usize {
    return MAX;
}