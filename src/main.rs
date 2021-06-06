use std::{u32, u64};

fn main() {
    println!("sum of multiples of 3 or 5 under 1000: {}", problem_1());
    println!(
        "sum of even values of fibonacci sequence under 4 million: {}",
        problem_2()
    );
}

fn problem_2() -> u64 {
    // sum of even values of fibonacci sequence under 4 million:
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;
    let mut result = 0;
    while result < 4000000 {
        result = a + b;
        a = b;
        b = result;

        // check a number is even
        if (result & 1) == 0 {
            sum += result;
        }
    }

    sum
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
