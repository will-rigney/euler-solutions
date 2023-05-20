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