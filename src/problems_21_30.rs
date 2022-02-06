use std::{
    char,
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};

use crate::utils::*;

/// sum of amicable numbers under 10000
pub fn problem_21() -> u64 {
    // list of primes
    let mut primes = vec![2, 3];

    // cache to hold previously calculated aliquot sums
    let mut cache = HashMap::<i32, i32>::new();

    // aliquot sum
    let mut s = |n: i32| {
        // hardcode s(1) to avoid divide by 0
        if n == 1 {
            return 1;
        }
        // check cache
        if let Some(res) = cache.get(&n) {
            return *res;
        }

        // prime factors of n
        let factors = factorise(n, &mut primes);

        // divisors of n formula
        let result: i32 = factors
            .iter()
            .map(|(p, a)| (p.pow((a + 1) as u32) - 1) / (p - 1))
            .product();
        // proper divisors of n
        let result = result - n;

        // store result in cache
        cache.insert(n, result);

        result
    };

    let mut sum = 0;

    for i in 2..10_000 {
        let t = s(i);
        // if this is an amicable number
        if i != t && i == s(t) {
            sum += i;
        }
    }
    sum as u64
}

/// total of name scores for names in 'p022_names.txt'
pub fn problem_22() -> i32 {
    // todo: better error handling for individual problems
    let names = fs::read("res/p022_names.txt").expect("error opening names.txt!");
    // use byte slices
    let mut sorted_names = BTreeMap::<&[u8], i32>::new();
    // split input by ','
    for name in names.split(|i| *i == b',') {
        // calculate alphabetical value
        // ignores "s as they're less than 64 in byte form
        let alphabetical_value = name.iter().map(|c| 0.max(*c as i32 - 64)).sum();
        // insert with name as key (sort for free)
        sorted_names.insert(name, alphabetical_value);
    }
    // find the result
    sorted_names
        .iter()
        .enumerate()
        .map(|(index, (_, value))| (index as i32 + 1) * value)
        .sum()
}

/// sum of positive integers that can't be expressed as sum of two abundant numbers
pub fn problem_23() -> i32 {
    // list of primes
    let mut primes = vec![2, 3];

    // reuse aliquot sum from problem 21
    // todo: move into utils w primes kept hidden in a struct
    // bit of a spicy invariant in that reliance on state means
    // arguments must be provided lexicographically to ensure primes found
    let mut s = |n: i32| {
        // hardcode s(1) to avoid divide by 0
        if n == 1 {
            return 1;
        }
        // prime factors of n
        let factors = factorise(n, &mut primes);
        // divisors of n formula
        let result: i32 = factors
            .iter()
            .map(|(p, a)| (p.pow((a + 1) as u32) - 1) / (p - 1))
            .product();
        // proper divisors of n
        result - n
    };

    let mut abundant = HashSet::<i32>::with_capacity(4994);

    // problem states provable limit is 28123, but it's actually 20161
    let limit = 20161;

    // sum of non-abundant sums
    let mut sum = 0;

    for i in 1..=limit {
        let mut should_add = true;
        for j in &abundant {
            // if i - j is not also abundant
            if abundant.contains(&(i - j)) {
                // is abundant sum, should not be added
                should_add = false;
                break;
            }
        }
        if should_add {
            sum += i;
        }
        // check if this number is abundant itself
        if s(i) > i {
            abundant.insert(i);
        }
    }
    sum
}

/// millionth lexicograghic permutation of 0-9
pub fn problem_24() -> String {
    /// calculate factorial basic way
    fn factorial(n: i32) -> i32 {
        (1..=n).product()
    }

    let mut numbers: Vec<u32> = (0..=9).collect();
    let mut n = 1_000_000 - 1;
    let mut result = "".to_string();
    let len = numbers.len() as i32;

    for digit in (1..len).rev() {
        let factorial = factorial(digit);
        let index = n / factorial;
        let digit = numbers.remove(index as usize);
        let char = char::from_digit(digit, 10).unwrap();
        result.push(char);
        n -= index * factorial;
    }
    let digit = numbers.pop().unwrap();
    let char = char::from_digit(digit, 10).unwrap();
    result.push(char);
    result
}

/// index of first term in fibonacci sequence to contain 1000 digits
pub fn problem_25() -> i32 {
    let num_digits = 1000.0;
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;

    // take base10 log of both sides of Binet's formula & solve for n to find index
    ((5.0_f32.log10() / 2.0 + num_digits - 1.0) / phi.log10()).ceil() as i32
}

/// d < 1000 for which 1/d has longest recurring cycle
pub fn problem_26() -> i32 {
    fn reciprocal_recurrence_length(d: i32) -> i32 {
        let mut result = 0;
        let mut value = 1;
        // repeating portion of remainder will be < d digits
        // holds number of repeating digits calculated as yet for given dividend
        let mut remainders = vec![0; d as usize];
        // while we haven't already calculated remainder for given value (i.e. hasn't yet repeated)
        while remainders[value as usize] == 0 {
            // update number of divisors
            remainders[value as usize] = result;
            // move to next column
            value *= 10;
            value %= d;
            result += 1;
        }
        // final result is number of iterations - initial remainder
        result - remainders[value as usize]
    }
    let mut result = 0;
    let mut max_length = 0;
    for d in (2..1000).rev() {
        // number of digits will never exceed d so we can short circuit
        if max_length > d {
            break;
        }
        let length = reciprocal_recurrence_length(d);
        if length > max_length {
            max_length = length;
            result = d;
        }
    }
    result
}

/// product of coefficients for quadratics of form n^2 + an + b producing most primes
pub fn problem_27() -> i64 {
    // prime seive from problem 10 to get all primes up to 16273
    // from brute force largest prime we see is 16273
    const MAX_PRIME: usize = 16274;
    const BOUND: usize = MAX_PRIME - 1;
    let sqrt = (MAX_PRIME as f32).sqrt() as usize + 1;

    let mut seive = [true; MAX_PRIME];
    seive[0] = false;
    seive[1] = false;

    // seive all the primes
    for i in 2..=sqrt {
        if seive[i] {
            let mut j = i * 2;
            while j < BOUND {
                seive[j as usize] = false;
                j += i;
            }
        }
    }

    let mut max_count = 0;
    let mut product = 0;
    for a in -999..1000 {
        for b in -1000..=1000 {
            let mut n: i64 = 0;
            loop {
                let result = (n.pow(2) + a * n + b).abs();
                if result > MAX_PRIME as i64 || !seive[result as usize] {
                    break;
                }
                n += 1;
            }
            if n > max_count {
                max_count = n;
                product = a * b;
            }
        }
    }
    product
}

/// sum of diagonals of 1001 x 1001 number spiral
pub fn problem_28() -> i32 {
    const MAX_LENGTH: i32 = 1001;
    // length of the side of the square
    let mut sum = 1;
    let mut length = 1;
    // last diagonal we visited on previous iteration
    let mut last_diagonal = 1;
    // start with second iteration
    while length < MAX_LENGTH {
        // we are summing numbers on the diagonals
        for _ in 0..4 {
            last_diagonal += length + 1;
            sum += last_diagonal;
        }

        // length should go up by 2 each time (2 new corners)
        length += 2;
    }
    sum
}

/// distinct terms in sequence generated by a^b for a, b in range 2-100 inclusive
pub fn problem_29() -> i32 {
    let mut terms = Vec::<f64>::new();
    // use floats to avoid integer overflow
    for a in 2..=100 {
        for b in 2..=100 {
            let pow = (a as f64).powf(b as f64);
            terms.push(pow)
        }
    }
    // floats dont't implement Ord
    terms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    terms.dedup();
    terms.len() as i32
}

/// sum of numbers that can be written as sum of 5th powers of their digits
pub fn problem_30() -> i32 {
    // need to find an upper bound
    // 9^5 = 59049, n >= 5
    // 7 * 9^5 = 413343 (not possible with 7 digits), n <= 6
    let mut numbers = Vec::<i32>::new();

    // iterate over every number to upper bound
    for number in 2..1000000 {
        let mut n: i32 = number;
        let mut sum = 0;
        while n != 0 {
            let digit = n % 10;
            sum += digit.pow(5);
            n /= 10;
        }
        if sum == number {
            numbers.push(number);
        }
    }
    numbers.iter().sum()
}
