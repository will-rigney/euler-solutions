use std::{
    collections::{BTreeMap, HashMap},
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
        /// increment the count for a given prime (or initialise to 1)
        fn inc(p: i32, factors: &mut BTreeMap<i32, i32>) {
            if let Some(count) = factors.get_mut(&p) {
                *count += 1;
            } else {
                factors.insert(p, 1);
            }
        }

        let mut factors = BTreeMap::new();
        let mut n = n;

        // loop while n is not fully factorised
        'outer: loop {
            for p in primes.iter() {
                // if n is an already seen prime, factorisation is complete
                if n == *p {
                    inc(*p, &mut factors);
                    return factors;
                }
                // p is a divisor (prime factor) of n
                if n % *p == 0 {
                    inc(*p, &mut factors);
                    n /= *p;
                    continue 'outer;
                }
            }
            // current n must be prime, add to list & return
            primes.push(n);
            inc(n, &mut factors);
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
    // use string slices
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
