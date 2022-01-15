use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};

/// sum of amicable numbers under 10000
pub fn problem_21() -> u64 {
    // list of primes
    let mut primes = vec![2, 3];

    // cache to hold previously calculated aliquot sums
    let mut cache = HashMap::<i32, i32>::new();

    /// find prime factors for a given n
    /// optimised by / requires passing a previously computed list of primes up to sqrt(n)
    fn factorise(n: i32, primes: &mut Vec<i32>) -> BTreeMap<i32, i32> {
        let mut factors = BTreeMap::new();
        let mut n = n;
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
    // todo: other version has been updated to use different api

    // list of primes
    let mut primes = vec![2, 3];
    // reuse aliquot sum from problem 21

    /// find prime factors for a given n
    /// optimised by / requires passing a previously computed list of primes up to sqrt(n)
    fn factorise(n: i32, primes: &mut Vec<i32>) -> BTreeMap<i32, i32> {

        let mut factors = BTreeMap::new();
        let mut n = n;

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

    // aliquot sum
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

    let mut abundant = HashSet::<i32>::new();

    // problem states provabe limit is 28123, but it's actually 20161
    let limit = 20161;

    // sum of non-abundant sums
    let mut sum = 0;

    for i in 1..=limit {
        let mut should_add = true;
        for j in &abundant {
            // if i - j is not also abundant
            if abundant.contains(&(i-j)) {
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
