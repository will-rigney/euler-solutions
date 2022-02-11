use std::collections::HashSet;

/// number of ways of making £2 in UK currency
pub fn problem_31() -> i32 {
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
    *solution.last().unwrap()
}

/// sum of products whose multiplicand/multiplier/product can be written as 1 through 9 pandigital
pub fn problem_32() -> u64 {
    /// check if a given integer input is pandigital
    /// n.b. only actually works on unsigned input
    fn is_pandigital(n: u64) -> bool {
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
        // product must be 4 digits, max multipier can't exceed 10000 / n + 1
        let high_mult = 10000 / n + 1;
        for m in low_mult..high_mult {
            let product = n * m;
            let result = concat_int(concat_int(n, m), product);
            if is_pandigital(result) {
                products.insert(product);
            }
        }
    }
    products.iter().sum()
}

/// find denominator in lowest terms of product of curious fractions
pub fn problem_33() -> i32 {
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
    result_d / result_n
}

/// sum of numbers equal to sum of factorials of their digits
pub fn problem_34() -> u64 {
    // we cheat and know a reasonable tight bound from previous run
    const MAX: usize = 50_000;

    // factorials of digits 0-9
    let mut factorials = [0; 10];
    factorials[0] = 1;
    factorials[1] = 1;
    factorials[2] = 2;

    for i in 3..=9 {
        factorials[i as usize] = i * factorials[(i - 1) as usize];
    }
    (3..(MAX as u64))
        .into_iter()
        .filter(|n| {
            // find sum of factorials of digits
            let mut sum = 0;
            let mut m = *n;
            while m > 0 {
                let d = m % 10;
                // add digit factorial to sum
                sum += factorials[d as usize];
                // move to next digit
                m /= 10;
            }
            sum == *n
        })
        .sum()
}
