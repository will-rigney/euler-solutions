use crate::{Int, utils::is_pandigital};
use std::collections::{BTreeSet, HashSet};

/// number of ways of making £2 in UK currency
pub fn problem_31() -> Int {
    const TARGET: usize = 200;
    // set of coins
    const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    // dynamic programming solution

    // table of each ways index can be made
    let mut solution = [0; TARGET + 1];

    // for 0, there is only 1 way (no coins)
    solution[0] = 1;

    // populate first for 1p, then 2p, for each coin up to max
    // (aka solve for each subset of coins incrementally)
    for coin in COINS {
        // update table incrementally for n > coin value
        for n in coin..=TARGET {
            solution[n] += solution[n - coin];
        }
    }
    let result = *solution.last().unwrap();
    result as Int
}

/// sum of products whose multiplicand/multiplier/product can be written as 1 through 9 pandigital
pub fn problem_32() -> Int {

    /// concat a to b, e.g. 123 + 456 = 123456
    fn concat_int(a: u64, b: u64) -> u64 {
        fn length(n: u64) -> u32 {
            let mut n = n;
            let mut count = 0;
            while n > 0 {
                n /= 10;
                count += 1;
            }
            count
        }
        a * (10_u64.pow(length(b))) + b
    }

    // now check if various combinations are pandigital
    let mut products = HashSet::new();
    // only count 1 & 2 digit multiplicands
    for n in 2..100 {
        // n is 1 digit needs 4 digit multiplier to make 4 digit result
        // n is 2 digits needs 3 digit multiplier to make 4 digit result
        let low_mult = if n < 10 { 1234 } else { 123 };
        // product must be 4 digits, max multiplier can't exceed 10000 / n + 1
        let high_mult = 10000 / n + 1;
        for m in low_mult..high_mult {
            let product = n * m;
            let result = concat_int(concat_int(n, m), product);
            if is_pandigital(result) {
                products.insert(product);
            }
        }
    }
    let result: u64 = products.iter().sum();
    result as Int
}

/// find denominator in lowest terms of product of curious fractions
pub fn problem_33() -> Int {
    let mut result_n = 1;
    let mut result_d = 1;

    // d >= 2 so fraction < 1
    for d in 2..=9 {
        for n in 1..d {
            let fraction = n as f32 / d as f32;
            // try and find curious fractions
            // c up to d will be on left side of numerator
            for c in 1..d {
                if (10 * c + n) as f32 / (10 * d + c) as f32 == fraction {
                    result_n *= n;
                    result_d *= d;
                }
            }
            // c >= d will be right side of numerator
            for c in d..=9 {
                if (10 * n + c) as f32 / (10 * c + d) as f32 == fraction {
                    result_n *= n;
                    result_d *= d;
                }
            }
        }
    }
    // return simplified denominator of the result
    let result = result_d / result_n;
    result as Int
}

/// sum of numbers equal to sum of factorials of their digits
pub fn problem_34() -> Int {
    // we cheat and know a reasonable tight bound from previous run
    const MAX: usize = 50_000;

    // factorials of digits 0-9
    let mut factorials = [0; 10];
    factorials[0] = 1;
    factorials[1] = 1;
    factorials[2] = 2;

    for i in 3..=9 {
        factorials[i] = i * factorials[i - 1];
    }
    let result: usize = (3..MAX)
        .into_iter()
        .filter(|n| {
            // find sum of factorials of digits
            let mut sum = 0;
            let mut m = *n;
            while m > 0 {
                let d = m % 10;
                // add digit factorial to sum
                sum += factorials[d];
                // move to next digit
                m /= 10;
            }
            sum == *n
        })
        .sum();
    result as Int
}

/// number of circular primes under one million
pub fn problem_35() -> Int {
    // use prime seive to find all primes up to max
    const MAX: usize = 1_000_000;
    let mut seive = [true; MAX + 1];
    crate::utils::seive(&mut seive, MAX + 1);

    // number of circular primes
    let mut n = 0;

    let mut primes: BTreeSet<i32> = seive
        .iter()
        .enumerate()
        .filter_map(|(i, p)| if *p { Some(i as i32) } else { None })
        .collect();

    'outer: while let Some(p) = primes.pop_first() {
        let mut c = 1;
        let len = (p as f32).log10() as i32 + 1;
        let mut p2 = p;

        for _ in 1..len {
            // last digit
            let d = p2 % 10;
            // move everything one digit right
            p2 /= 10;
            // add digit back on the left side
            p2 += d * 10_i32.pow(len as u32 - 1);

            // if the permutation is equal to the original prime then it's circular
            if p2 == p {
                n += 1;
                continue 'outer;
            }

            if primes.remove(&p2) {
                // this is circular, increment temporary count
                c += 1;
            } else {
                // these are not circular primes
                continue 'outer;
            }
        }
        // if all permutations are prime
        if c == len {
            n += c;
        }
    }
    n as Int
}

/// sum of numbers less than 1000000 that are palindromes in base 10 & base 2
pub fn problem_36() -> Int {
    const MAX: i32 = 1_000_000;
    /// checks if a string is a palindrome, e.g. same as its reversed form
    fn is_palindrome(s: &str) -> bool {
        s == s.chars().rev().collect::<String>()
    }

    // palindromes in base 2 must not have leading zeros, so must be odd numbers
    let mut n = 1;
    let mut sum = 0;
    while n < MAX {
        let decimal = n.to_string();
        // check if it's a palindrome
        let binary = format!("{:b}", n);
        // check if it's a palindrome
        if is_palindrome(&decimal) && is_palindrome(&binary) {
            sum += n;
        }
        n += 2;
    }
    sum as Int
}

/// sum of primes that are truncatable in both directions
pub fn problem_37() -> Int {
    // max prime is 739397
    const MAX: usize = 739398;

    // use prime seive to find all primes up to max
    let mut seive = [true; MAX + 1];
    crate::utils::seive(&mut seive, MAX);

    // problem states there are 11 such numbers
    const N_PRIMES: i32 = 11;
    // number of truncatable primes found so far
    let mut primes_found = 0;
    let mut sum = 0;
    let mut n = 11;

    while primes_found < N_PRIMES {
        if seive[n] {
            // this number is prime
            let len = (n as f32).log10() as i32 + 1;
            let mut is_truncatable = true;
            // check if truncated versions are also prime
            for i in 1..len {
                // n.b. 10.pow function could be const for known values of len
                // left side
                let l = n % 10_usize.pow((len - i) as u32);
                // right side
                let r = n / 10_usize.pow(i as u32);
                if !seive[l] || !seive[r] {
                    is_truncatable = false;
                    break;
                }
            }

            if is_truncatable {
                sum += n;
                primes_found += 1;
            }
        }
        n += 2;
    }
    sum as Int
}

/// largest 1 to 9 pandigital formed as concatenated product of integer with (1, ..., n)
pub fn problem_38() -> Int {
    // todo: definitely some cool maths around that could speed this up
    let mut largest_pandigital = 0;
    // concatenating x * 1 & x * 2 must be no greater than 9 digits
    // so x is max 4 digits
    for x in 1..9999 {
        let mut concatenated_result: u64 = x;
        let mut multiplicand = 2;

        while concatenated_result.checked_ilog10().unwrap() + 1 < 9 {
            let result = x * multiplicand;
            multiplicand += 1;
            // concatenate
            let shift_multiplier = 10_u64.pow(result.checked_ilog10().unwrap() + 1);
            concatenated_result *= shift_multiplier;
            concatenated_result += result;
        }

        // ok now we have a number at least 9 digits
        // check it's exactly 9
        if concatenated_result.checked_ilog10().unwrap() > 9 {
            continue;
        }

        // now check if it's pandigital and bigger than the current biggest
        // todo: check order of these conditions
        if is_pandigital(concatenated_result) && concatenated_result > largest_pandigital {
            largest_pandigital = concatenated_result;
        }
    }

    largest_pandigital as Int
}

/// right triangle perimeter p with most solutions for a^2 + b^2 = c^2
pub fn problem_39() -> Int {
    // a^2 + b^2 = c^2
    // replace c term in terms of p
    // a^2 + b^2 = (p - a - b)^2
    // a^2 + b^2 = p^2 - pa - pb - ap + a^2 + ab - bp + ba + b^2
    // 0 = p^2 - 2pa - 2pb + 2ab

    // 2pb - 2ab = p^2 - 2pa
    // 2b(p - a) = p^2 - 2pa
    // b = (p^2 - 2pa) / 2(p - a)

    fn calculate_b(p: i64, a: i64) -> i64 {
        let numerator = p * p - 2 * a * p;
        let denominator = 2 * (p - a);
        numerator / denominator // n.b. this has to be an integer! no rounding!
    }

    let mut max_solutions = 0;
    let mut max_p = 0;

    // only test for even values of p
    for p in (2..1000).step_by(2) {
        // note: solutions should be distinct (doesn't affect final answer)
        let mut n_solutions = 0;
        for a in 1..p {
            // calculate b
            let b = calculate_b(p, a);
            // no negative b (problem somewhere else)
            if b <= 0 {
                continue;
            }
            // check c is square
            let c_squared = (a * a + b * b) as f64;
            let c = c_squared.sqrt();
            // make sure c is integer
            if c.fract() != 0.0 {
                continue;
            }

            n_solutions += 1;
        }

        if n_solutions > max_solutions {
            max_p = p;
            max_solutions = n_solutions;
        }
    }
    max_p
}

/// d1 * d10 * d100 * d1000 * d10000 * d1000000 of fractional part of Champernowne's constant
pub fn problem_40() -> i64 {
    // can this string be created at compile time?
    // every problem should be solved at compile time anyway
    let string: String = {
        let mut string = String::with_capacity(600000);
        for i in 1..1000000 {
            string.push_str(&i.to_string());
        }
        string
    };

    // indices of digits
    let digits: [usize; 7] = [0, 9, 99, 999, 9999, 99999, 999999];

    digits
        .iter()
        .map(|n| {
            let char = string.chars().nth(*n).unwrap(); // something about an error
            char.to_digit(10).unwrap() as i64
        })
        .product()
}
