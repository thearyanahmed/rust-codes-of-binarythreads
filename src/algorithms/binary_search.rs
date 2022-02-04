use rand::{self,Rng};

const MAX : usize = 10000;

pub fn binary_search(array : &mut [i64], target: i64) -> i32 {
    println!("hello from binary search mod target {}",target);

    return 1;
}

pub fn run() {

    let mut rng = rand::thread_rng();

    let mut array: [i64; MAX] = [0; MAX];

    for i in 0..MAX {
        array[i] = rng.gen::<i64>();
    }

    let val : i64 = 1000000;

    let index = binary_search(&mut array,val);

    println!("found index {} for {}",index, val)
}