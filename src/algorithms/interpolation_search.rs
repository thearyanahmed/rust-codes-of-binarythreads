#![allow(non_snake_case)]
use rand::{self,Rng};
use crate::utils::array as arrayUtil;

const MAX : usize = 1000000;

pub fn run() {
    let mut ranger = rand::thread_rng();

    let mut array = arrayUtil::randomInt32Array();

    array.sort();
    let length = array.len();

    let target : i32 = array[ranger.gen_range(0..MAX)];
    let index = interpolationSearch(&mut array,0,(length - 1) as i32, target);

    println!("array[0] = {} \narray[last] = {}\narray length = {}\ntarget {}\nfound at index {}\n",array[0], array[MAX - 1],length, target,index)
}

pub fn interpolationSearch(array: &mut [i32; MAX], low: i32, high: i32, target: i32) -> i32 {
    if low <= high && target >= array[low as usize] && target <= array[high as usize] {
        let position = low + (((high - low) / (array[high as usize] - array[low as usize])) * (target - array[low as usize]));

        if array[position as usize] == target {
            return target;
        }

        if array[position as usize] < target {
            return interpolationSearch(array,position + 1, high, target);
        }

        return interpolationSearch(array,low, position - 1, target);
    }

    return -1;
}