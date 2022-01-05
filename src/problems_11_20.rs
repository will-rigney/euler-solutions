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
            .map(|i| (n + 1 - i) as f64 / i as f64)
            .product::<f64>() as i64
    }
    binomial(2 * n, n)
}

/// sum of digits in 2^1000
pub fn problem_16() -> u64 {
    // previous runs say 302 is enough
    let mut digits = [0; 302];

    // start with 2^0
    digits[0] = 1;

    // highest seen column (skip evaluating past this point)
    let mut highest_column = 0;

    // multiply by 2 999 times
    for _ in 1..=1000 {
        // current value of the carry
        let mut carry = 0;
        // index of current digit
        let mut i = 0;

        while i <= highest_column {
            // current digit * 2 + carry
            let product = 2 * digits[i] + carry;
            // update digit
            digits[i] = product % 10;
            // update carry
            carry = product / 10;
            // next column
            if i == highest_column && carry > 0 {
                highest_column += 1;
            }
            // increment index
            i += 1;
        }
    }
    digits.iter().sum()
}

/// number of characters in 1-1000 (inclusive) written as words
pub fn problem_17() -> u64 {
    // numbers 1-9
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    // 10 - 19
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    // 20 - 90 by 10
    let decades = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

    let hundred = "hundred";
    let and = "and";

    let mut count = 0;

    // total letters in numbers 1-99
    let mut hundred_count = 0;

    // 1-9
    for number in numbers {
        hundred_count += number.len() as u64;
    }

    // 10 - 19
    for teen in teens {
        hundred_count += teen.len() as u64;
    }

    for decade in decades {
        hundred_count += decade.len() as u64;
        for number in numbers {
            hundred_count += (decade.len() + number.len()) as u64;
        }
    }

    // 1 - 99
    count += hundred_count;

    for number in numbers {
        // add "<number> hundred"
        // e.g. "one hundred"
        count += (number.len() + hundred.len()) as u64;
        // add "<number> hundred and 99x"
        // e.g. "one hundred and"
        let hundreds = (number.len() + hundred.len() + and.len()) as u64;

        // number of digits in 1-99
        count += hundred_count;
        // 99 * "<number> hundred and"
        count += hundreds * 99;
    }

    // 1000
    count += "onethousand".len() as u64;

    count
}

/// maximum top to bottom path in 15 row triangle
pub fn problem_18() -> u64 {
    let mut triangle = [
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 04, 82, 47, 65],
        vec![19, 01, 23, 75, 03, 34],
        vec![88, 02, 77, 73, 07, 63, 67],
        vec![99, 65, 04, 28, 06, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
    ];

    // solve problem by replacing each cell with the optimum solution from
    // the two cells below (remember DP?)
    let row = triangle.len() - 1;
    for i in (0..row).rev() {
        let row_length = triangle[i].len();
        for j in 0..row_length {
            let max = (triangle[i + 1][j]).max(triangle[i + 1][j + 1]);
            triangle[i][j] += max;
        }
    }
    // top of the triangle is the answer
    triangle[0][0]
}

/// number of sundays in 20th century
pub fn problem_19() -> u64 {
    // days, expressed as ints
    #[derive(PartialEq, Eq, Debug)]
    enum Day {
        Monday = 0,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // could use strum for this but I won't
    impl From<u64> for Day {
        fn from(d: u64) -> Self {
            match d {
                0 => Day::Monday,
                1 => Day::Tuesday,
                2 => Day::Wednesday,
                3 => Day::Thursday,
                4 => Day::Friday,
                5 => Day::Saturday,
                6 => Day::Sunday,
                _ => panic!("trying to create Day from int out of bounds {}", &d),
            }
        }
    }

    #[derive(Debug)]
    enum Months {
        January,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }

    impl Months {
        fn n_days(&self, is_leap_year: bool) -> u64 {
            match self {
                Self::February => {
                    if is_leap_year {
                        29
                    } else {
                        28
                    }
                } // add one for leap years
                Self::September | Self::April | Self::June | Self::November => 30,
                _ => 31,
            }
        }
    }

    // could also use strum for this
    let months = [
        Months::January,
        Months::February,
        Months::March,
        Months::April,
        Months::May,
        Months::June,
        Months::July,
        Months::August,
        Months::September,
        Months::October,
        Months::November,
        Months::December,
    ];

    let mut count = 0;

    // 1st january 1901 was a tuesday
    let mut first_of_month = Day::Tuesday;

    // years from 1901 - 2000
    for year in 1..=100 {
        // leap year
        let is_leap_year = year % 4 == 0;
        for month in &months {
            // check first of month
            if first_of_month == Day::Sunday {
                count += 1;
            }
            // number of days in month
            let n_days = month.n_days(is_leap_year);
            // update to first of next month
            let day_index = first_of_month as u64;
            first_of_month = Day::from((day_index + n_days) % 7);
        }
    }
    count
}

/// sum of digits in the number 100!
pub fn problem_20() -> u64 {
    // same as problem 16 - use long multiplication
    // 158 digits is enough
    let mut digits = [0; 158];

    // start with 1
    digits[0] = 1;

    // highest seen column (skip evaluating past this point)
    let mut highest_column = 0;

    // 100 factorial long multiplication
    for n in (1..100).rev() {
        // current value of the carry
        let mut carry = 0;
        // index of current digit
        let mut i = 0;

        while i <= highest_column {
            // current digit * 2 + carry
            let product = n * digits[i] + carry;
            // update digit
            digits[i] = product % 10;
            // update carry
            carry = product / 10;
            // next column
            if i == highest_column && carry > 0 {
                highest_column += 1;
            }
            // increment index
            i += 1;
        }
    }
    digits.iter().sum()
}
