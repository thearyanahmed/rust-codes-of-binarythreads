use rand::{self,Rng};

const MAX : usize = 10000;
const MAX_VALUE : u32 = 10000000;

pub fn binary_search(array : &mut [u32], target: u32) -> u32 {
    println!("hello from binary search mod target {}",target);

    for (_, v) in array.iter().enumerate() {
        println!("v {} ", v);
    }
    return 1;
}

pub fn run() {

    let mut rng = rand::thread_rng();

    let mut array: [u32; MAX] = [0; MAX];

    for i in 0..MAX {
        array[i] = rng.gen_range(0..MAX_VALUE);
    }

    let val : u32 = 1000000;

    let index = binary_search(&mut array,val);

    println!("found index {} for {}",index, val)
}