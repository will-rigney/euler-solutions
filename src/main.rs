use std::u64;

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
}

fn problem_4() -> i64 {
    // palindrome test
    fn is_palindrome(n: i64) -> bool {
        let mut a = n;
        let mut b = 0;
        // for each of the digits in the input
        while a > 0 {
            // t is the next digit
            let t = a % 10;
            // move current digits up one and add new digit on the right
            b = (b * 10) + t;
            // move to next digit
            a /= 10;
        }
        // if reversed is equal to original it is palindrome
        n == b
    }

    // brute force
    // having a fast pc and a nice compiler makes this easier
    let mut result = 0;
    for x in (100..999).rev() {
        for y in (100..999).rev() {
            let product = x * y;
            if product > result && is_palindrome(product) {
                result = product;
            }
        }
    }
    result
}

fn problem_3() -> i64 {
    let n = 600851475143;

    // prime test
    fn is_prime(n: i64) -> bool {
        // get the truncated square root
        let mut m = (n as f64).sqrt() as i64;

        while m > 1 {
            if n % m == 0 {
                // not prime
                return false;
            }
            m -= 1;
        }
        true
    }

    let sqrt = (n as f64).sqrt() as i64;

    let mut i = 2;
    let mut highest_prime_lower_divisor = 1;

    // while i is less than the square root of n
    while i < sqrt {
        // if n is divisible by i
        if n % i == 0 {
            // if i is prime we save it for later
            if is_prime(i) {
                highest_prime_lower_divisor = i;
            }
            // get the divisor
            let j = n / i;
            // check if it's prime
            if is_prime(j) {
                // this must be the largest prime factor
                return j;
            }
        }

        // increment i
        i += 1;
    }
    // return the highest prime divisor we had seen so far
    highest_prime_lower_divisor
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