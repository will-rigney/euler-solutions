mod problems_11_20;
mod problems_1_10;
use problems_11_20::*;
use problems_1_10::*;

fn main() {
    println!("sum of multiples of 3 or 5 under 1000: {}", problem_1());
    println!(
        "sum of even values of fibonacci sequence under 4 million: {}",
        problem_2()
    );
    println!("largest prime factor of 600851475143: {}", problem_3());
    println!(
        "largest palindrome number that is product of two 3 digit numbers: {}",
        problem_4()
    );
    println!(
        "smallest positive number divisible by all numbers from 1 to 20: {}",
        problem_5()
    );
    println!(
        "difference between sum of squares and square of sum of numbers from 1 to 100: {}",
        problem_6()
    );
    println!("10001st prime: {}", problem_7());
    println!(
        "product of 13 adjacent digits in the 1000 digit number with the greatest product: {}",
        problem_8()
    );
    println!(
        "product of pythagorean triple whose sum is 1000: {}",
        problem_9()
    );
    println!("sum of all primes under 2 million: {}", problem_10());

    println!(
        "greatest product of 4 numbers in each direction of 20 x 20 array: {}",
        problem_11()
    );

    println!(
        "first triangle number with over 500 divisors: {}",
        problem_12()
    );
}
