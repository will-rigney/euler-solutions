use std::{u32, u64};

fn main() {
    println!("sum of multiples of 3 or 5 under 1000: {}", problem_1());
}

fn problem_1() -> u64 {
    let mut sum = 0;
    // dumb brute force way
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
