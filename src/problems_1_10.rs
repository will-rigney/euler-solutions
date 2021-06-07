pub fn problem_10() -> i64 {
    // very similar to solution to problem 7
    // test if n can be divided by any of the entries in `primes`
    fn is_prime(n: i64, primes: &[i64]) -> bool {
        // get truncated square
        let sqrt = (n as f64).sqrt() as i64;
        for i in primes {
            // not a prime
            if n % i == 0 {
                return false;
            }
            // only test up to the square root
            if i > &sqrt {
                return true;
            }
        }
        true
    }

    let max = 2000000;

    // current number
    let mut n = 3;

    // keep a list of primes we've seen in our gigabytes of ram
    let mut primes = vec![3]; // always odd numbers

    // skip the number 2 for less divisions
    while n < max {
        // next odd number
        n += 2;
        // check if i n is prime by dividing it by all of our other primes
        if is_prime(n, &primes) {
            // add it to the list
            primes.push(n);
        }
    }
    // add the first prime
    primes.push(2);
    primes.iter().sum()
}

pub fn problem_9() -> i64 {
    let mut squares = vec![];

    // add each square
    let mut n: i64 = 1;

    while n < 1000 {
        squares.push(n.pow(2));
        n += 1;
    }

    // consider each possible c from largest to smallest
    for (c, c_sq) in squares.iter().enumerate().rev() {
        // only a < c
        for (b, b_sq) in squares.iter().enumerate().take(c) {
            // we're using the index which is 0 indexed
            let b = (b + 1) as i64;
            let c = (c + 1) as i64;

            let a = 1000 - c - b;

            // a must be positive
            if a < 0 {
                continue;
            }

            let a_sq = a.pow(2);

            // only 1 possible triple
            if *c_sq == a_sq + b_sq {
                return a * b * c;
            }
        }
    }
    panic!("triplet not found")
}

pub fn problem_8() -> i64 {
    // length of sub strings
    let len = 13;

    // the 1000 digit number with whitespace removed
    let n: String = "73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450"
        .split_whitespace()
        .collect();

    let mut result = 0;

    // for each segment of length 13
    for i in 0..(n.len() - len) {
        // get the string segment
        let segment = &n[i..(i + len)];

        // calculate the product
        let product = segment
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .product();

        if product > result {
            result = product;
        }
    }
    result
}

pub fn problem_7() -> i64 {
    // test if n can be divided by any of the entries in `primes`
    fn is_prime(n: i64, primes: &[i64]) -> bool {
        // get truncated square
        let sqrt = (n as f64).sqrt() as i64;
        for i in primes {
            // not a prime
            if n % i == 0 {
                return false;
            }
            // only test up to the square root
            if i > &sqrt {
                return true;
            }
        }
        true
    }

    // current number
    let mut n = 3;

    // keep a list of primes we've seen in our gigabytes of ram
    let mut primes = vec![3]; // always odd numbers

    // skip the number 2 for less divisions
    while primes.len() < 10001 - 1 {
        // next odd number
        n += 2;
        // check if i n is prime by dividing it by all of our other primes
        if is_prime(n, &primes) {
            // add it to the list
            primes.push(n);
        }
    }
    *primes.last().unwrap()
}

pub fn problem_6() -> i64 {
    let max = 100;

    // get the sum of the squares
    let sum_squares: i64 = (1..=max)
        .map(|x| x * x)
        .reduce(|a, b| a + b)
        .expect("couldn't sum squares");

    // get the square of the sum
    let square_sum: i64 = (1..=max)
        .reduce(|a, b| a + b)
        .expect("couldn't square sums")
        .pow(2);

    (square_sum - sum_squares).abs()
}

pub fn problem_5() -> i64 {
    // test if a number n is divisible by all numbers from 1 to max
    fn test_divisible(n: i64, max: i64) -> bool {
        for i in (2..max).rev() {
            if n % i != 0 {
                return false;
            }
        }
        true
    }

    // brute force
    let max = 20;
    let mut result = 20;
    loop {
        if test_divisible(result, max) {
            return result;
        } else {
            result += 20;
        }
    }
}

pub fn problem_4() -> i64 {
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

pub fn problem_3() -> i64 {
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

pub fn problem_2() -> u64 {
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

pub fn problem_1() -> u64 {
    let mut sum = 0;
    // dumb brute force way
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
