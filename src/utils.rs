/// module for utility functions used in solutions to multiple problems
use std::collections::BTreeMap;

/// find prime factors for a given n
/// optimised by / requires passing a previously computed list of primes up to sqrt(n)
pub fn factorise(n: i32, primes: &mut Vec<i32>) -> BTreeMap<i32, i32> {
    let mut factors = BTreeMap::new();
    let mut n = n;

    if n == 0 {
        return factors;
    }
    // loop while n is not fully factorised
    'outer: loop {
        for p in primes.iter() {
            // if n is an already seen prime, factorisation is complete
            if n == *p {
                *factors.entry(*p).or_insert(0) += 1;
                return factors;
            }
            // p is a divisor (prime factor) of n
            if n % *p == 0 {
                *factors.entry(*p).or_insert(0) += 1;
                n /= *p;
                continue 'outer;
            }
        }
        // current n must be prime, add to list & return
        primes.push(n);
        *factors.entry(n).or_insert(0) += 1;
        return factors;
    }
}

/// seive of Eratosthenes using slice of bools
// todo: consider impl on a slice maybe
pub fn seive(seive: &mut [bool], max: usize) {
    let sqrt = ((max - 1) as f32).sqrt() as usize + 1;
    // initial primes
    seive[0] = false;
    seive[1] = false;
    // seives all the primes by index
    for i in 2..=sqrt {
        if seive[i] {
            let mut j = i * 2;
            while j < max - 1 {
                seive[j] = false;
                j += i;
            }
        }
    }

}

/// check if a given integer input is pandigital
/// n.b. only actually works on unsigned input
pub fn is_pandigital(n: u64) -> bool {
    let mut n = n;
    let mut digits = 0;
    let mut length = 0;
    // iterate through each digit in n
    while n > 0 {
        // get the current digit
        let digit = n % 10;
        // 0 doesn't count
        if digit == 0 {
            return false;
        }
        // set corresponding bit for digit to 1
        let new_digits = digits | (1 << (digit - 1));
        // if digits unchanged (i.e. digit has repeated)
        if digits == new_digits {
            return false;
        }
        digits = new_digits;
        // next digit
        n /= 10;
        length += 1;
    }
    // only count results that have all digits 1-9
    if length < 9 {
        return false;
    }
    // all digit bits should be 1
    digits == (1 << length) - 1
}