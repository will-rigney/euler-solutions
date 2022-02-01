use crate::utils::*;

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
    // list of primes for factorisation
    let mut primes = vec![2, 3];

    // count the number of divisors for the input number using prime factorisation
    let mut count_divisors =
        |n: i32| -> i32 { factorise(n, &mut primes).values().map(|e| e + 1).product() };

    let mut triangle_number = 1;
    for i in 2.. {
        triangle_number += i;
        if count_divisors(triangle_number) > 500 {
            break;
        }
    }
    triangle_number
}

/// first 10 digits of sum of 100 50 digit numbers
pub fn problem_13() -> String {
    let array = [
        "37107287533902102798797998220837590246510135740250",
        "46376937677490009712648124896970078050417018260538",
        "74324986199524741059474233309513058123726617309629",
        "91942213363574161572522430563301811072406154908250",
        "23067588207539346171171980310421047513778063246676",
        "89261670696623633820136378418383684178734361726757",
        "28112879812849979408065481931592621691275889832738",
        "44274228917432520321923589422876796487670272189318",
        "47451445736001306439091167216856844588711603153276",
        "70386486105843025439939619828917593665686757934951",
        "62176457141856560629502157223196586755079324193331",
        "64906352462741904929101432445813822663347944758178",
        "92575867718337217661963751590579239728245598838407",
        "58203565325359399008402633568948830189458628227828",
        "80181199384826282014278194139940567587151170094390",
        "35398664372827112653829987240784473053190104293586",
        "86515506006295864861532075273371959191420517255829",
        "71693888707715466499115593487603532921714970056938",
        "54370070576826684624621495650076471787294438377604",
        "53282654108756828443191190634694037855217779295145",
        "36123272525000296071075082563815656710885258350721",
        "45876576172410976447339110607218265236877223636045",
        "17423706905851860660448207621209813287860733969412",
        "81142660418086830619328460811191061556940512689692",
        "51934325451728388641918047049293215058642563049483",
        "62467221648435076201727918039944693004732956340691",
        "15732444386908125794514089057706229429197107928209",
        "55037687525678773091862540744969844508330393682126",
        "18336384825330154686196124348767681297534375946515",
        "80386287592878490201521685554828717201219257766954",
        "78182833757993103614740356856449095527097864797581",
        "16726320100436897842553539920931837441497806860984",
        "48403098129077791799088218795327364475675590848030",
        "87086987551392711854517078544161852424320693150332",
        "59959406895756536782107074926966537676326235447210",
        "69793950679652694742597709739166693763042633987085",
        "41052684708299085211399427365734116182760315001271",
        "65378607361501080857009149939512557028198746004375",
        "35829035317434717326932123578154982629742552737307",
        "94953759765105305946966067683156574377167401875275",
        "88902802571733229619176668713819931811048770190271",
        "25267680276078003013678680992525463401061632866526",
        "36270218540497705585629946580636237993140746255962",
        "24074486908231174977792365466257246923322810917141",
        "91430288197103288597806669760892938638285025333403",
        "34413065578016127815921815005561868836468420090470",
        "23053081172816430487623791969842487255036638784583",
        "11487696932154902810424020138335124462181441773470",
        "63783299490636259666498587618221225225512486764533",
        "67720186971698544312419572409913959008952310058822",
        "95548255300263520781532296796249481641953868218774",
        "76085327132285723110424803456124867697064507995236",
        "37774242535411291684276865538926205024910326572967",
        "23701913275725675285653248258265463092207058596522",
        "29798860272258331913126375147341994889534765745501",
        "18495701454879288984856827726077713721403798879715",
        "38298203783031473527721580348144513491373226651381",
        "34829543829199918180278916522431027392251122869539",
        "40957953066405232632538044100059654939159879593635",
        "29746152185502371307642255121183693803580388584903",
        "41698116222072977186158236678424689157993532961922",
        "62467957194401269043877107275048102390895523597457",
        "23189706772547915061505504953922979530901129967519",
        "86188088225875314529584099251203829009407770775672",
        "11306739708304724483816533873502340845647058077308",
        "82959174767140363198008187129011875491310547126581",
        "97623331044818386269515456334926366572897563400500",
        "42846280183517070527831839425882145521227251250327",
        "55121603546981200581762165212827652751691296897789",
        "32238195734329339946437501907836945765883352399886",
        "75506164965184775180738168837861091527357929701337",
        "62177842752192623401942399639168044983993173312731",
        "32924185707147349566916674687634660915035914677504",
        "99518671430235219628894890102423325116913619626622",
        "73267460800591547471830798392868535206946944540724",
        "76841822524674417161514036427982273348055556214818",
        "97142617910342598647204516893989422179826088076852",
        "87783646182799346313767754307809363333018982642090",
        "10848802521674670883215120185883543223812876952786",
        "71329612474782464538636993009049310363619763878039",
        "62184073572399794223406235393808339651327408011116",
        "66627891981488087797941876876144230030984490851411",
        "60661826293682836764744779239180335110989069790714",
        "85786944089552990653640447425576083659976645795096",
        "66024396409905389607120198219976047599490197230297",
        "64913982680032973156037120041377903785566085089252",
        "16730939319872750275468906903707539413042652315011",
        "94809377245048795150954100921645863754710598436791",
        "78639167021187492431995700641917969777599028300699",
        "15368713711936614952811305876380278410754449733078",
        "40789923115535562561142322423255033685442488917353",
        "44889911501440648020369068063960672322193204149535",
        "41503128880339536053299340368006977710650566631954",
        "81234880673210146739058568557934581403627822703280",
        "82616570773948327592232845941706525094512325230608",
        "22918802058777319719839450180888072429661980811197",
        "77158542502016545090413245809786882778948721859617",
        "72107838435069186155435662884062257473692284509516",
        "20849603980134001723930671666823555245252804609722",
        "53503534226472524250874054075591789781264330331690",
    ];

    // long summation to figure it out without big nums
    let mut result = vec![];

    // carry from the previous digit
    let mut carry = 0;

    // iterate through digit columns
    for i in (0..array[0].len()).rev() {
        // todo: better errors
        let sum = array
            .iter()
            .map(|s| s.chars().nth(i).unwrap().to_digit(10).unwrap())
            .sum::<u32>()
            + carry;

        // set the current digit of result to the final digit of the sum
        result.push(sum % 10);

        // update the carry
        carry = sum / 10;
    }

    // add remaining digits of the carry
    while carry > 0 {
        result.push(carry % 10);
        carry /= 10;
    }

    // convert first 10 digits to string
    result
        .iter()
        .rev()
        .take(10)
        .map(|i| i.to_string())
        .reduce(|s, c| s + &c)
        .unwrap()
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

    const MAX: u64 = 1_000_000;

    // cache holds number of steps for each iteration we have seen
    let mut cache = vec![0; MAX as usize];
    cache[1] = 1;

    // longest length seen
    let mut length = 0;
    // starting number with longest length seen
    let mut result = 0;

    for i in 2..MAX {
        let mut n = i;
        let mut s = 0;
        // while sequence not complete and n greater than already computed value
        while n != 1 && n >= i {
            s += 1;
            n = collatz_step(n);
        }

        // then store the result in cache
        cache[i as usize] = s + cache[n as usize];

        // check if sequence is the best solution
        if cache[i as usize] > length {
            length = cache[i as usize];
            result = i;
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

    // multiply by 2 1000 times
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
        Monday,
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
