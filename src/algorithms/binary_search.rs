#![allow(non_snake_case)]
use rand::{self,Rng};
use crate::utils::array as arrayUtil;

const MAX : usize = 1000000;
// const MAX_VALUE : u32 = 10000000;

pub fn binary_search(array : &mut [i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if array[mid] == target {
            return mid as i32;
        }

        if target < array[mid] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    return -1;
}

pub fn run() {
    let mut ranger = rand::thread_rng();

    let mut array = arrayUtil::randomInt32Array();

    array.sort();

    let target : i32 = array[ranger.gen_range(0..MAX)];
    let index = binary_search(&mut array,target);

    println!("array[0] = {} \narray[last] = {}\narray length = {}\ntarget {}\nfound at index {}\n",array[0], array[MAX - 1],array.len(), target,index)
}