/// largest product in a hardcoded grid
pub fn problem_11() -> i32 {
    let array = [
        [08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
        [49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
        [81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
        [52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
        [22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
        [24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
        [32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
        [67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
        [24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
        [21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
        [78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
        [16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
        [86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
        [19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
        [04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
        [88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
        [04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
        [20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
        [20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
        [01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48],
    ];

    let mut greatest_product = 0;

    // iterate through the grid
    for (y, row) in array.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            // find product of four to the right
            if x + 3 < row.len() {
                let product = array[y][x] * array[y][x + 1] * array[y][x + 2] * array[y][x + 3];
                greatest_product = greatest_product.max(product);
            }

            // find product of four below
            if y + 3 < array.len() {
                let sum = array[y][x] * array[y + 1][x] * array[y + 2][x] * array[y + 3][x];
                greatest_product = greatest_product.max(sum);
            }

            // find product of four diagonally left
            if x > 3 && y + 3 < array.len() {
                let sum =
                    array[y][x] * array[y + 1][x - 1] * array[y + 2][x - 2] * array[y + 3][x - 3];
                greatest_product = greatest_product.max(sum);
            }

            // find product of four diagonally right
            if x + 3 < row.len() && y + 3 < array.len() {
                let sum =
                    array[y][x] * array[y + 1][x + 1] * array[y + 2][x + 2] * array[y + 3][x + 3];
                greatest_product = greatest_product.max(sum);
            }
        }
    }
    greatest_product
}

/// first triangle number with over 500 divisors
pub fn problem_12() -> i32 {
    // count the number of divisors for the input number (naive version)
    fn count_divisors(n: i32) -> i32 {
        let mut count = 0;
        for i in 1..((n as f32).sqrt() as i32) {
            if n % i == 0 {
                // don't count equal divisors twice
                count += if n / i == i { 1 } else { 2 }
            }
        }
        count
    }

    let mut triangle_number = 0;
    for i in 1.. {
        triangle_number += i;
        if count_divisors(triangle_number) > 500 {
            break;
        }
    }
    triangle_number
}

/// starting number of longest collatz sequence under one million
pub fn problem_14() -> u64 {
    /// returns next term in collatz sequence given current term `n`
    fn collatz_step(n: u64) -> u64 {
        if n & 0b1 == 1 {
            3 * n + 1
        } else {
            n / 2
        }
    }

    // highest starting number
    let mut result = 0;

    // highest number of terms seen
    let mut max = 0;

    for s in 1..1_000_000 {
        let mut n = s;
        let mut i = 0;
        while n != 1 {
            // increment number of terms
            i += 1;
            n = collatz_step(n);
        }
        if i > max {
            max = i;
            result = s;
        }
    }
    result
}

/// lattice paths in 20x20 grid
pub fn problem_15() -> i64 {

    // there are always n left movements & n down movements for n x n grid
    // i.e. for each given step, we can take one of n left movements or
    // one of n down movements

    // so solution is 2n choose n
    // (n choices then take whatevers remaining to complete the path)

    let n = 20;

    /// calculate the value of n choose k
    fn binomial(n: i64, k: i64) -> i64 {
        // sum from i=1 to smaller of k or n-k
        let k = k.min(n - k);
        (1..=k)
            // (n + 1 - i) / i
            .map(|i| {(n + 1 - i) as f64 / i as f64 })
            .product::<f64>() as i64
    }
    binomial(2 * n, n)
}
