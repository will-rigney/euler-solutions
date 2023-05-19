use once_cell::sync::Lazy;
use clap::Parser;

mod print;
mod problems_11_20;
mod problems_1_10;
mod problems_21_30;
mod problems_31_40;

use print::*;
use problems_11_20::*;
use problems_1_10::*;
use problems_21_30::*;
use problems_31_40::*;

mod utils;

#[cfg(test)]
mod test;

#[derive(Parser, Debug)]
#[clap(
    about = "Solutions to problems at https://projecteuler.net/ implemented in Rust.",
    author = "Will Rigney"
)]

struct Args {
    /// Specify the problem to solve; defaults to 0 which means all problems.
    #[clap(short, long, default_value_t = 0)]
    problem: usize,
    /// Censor the results (no spoilers!).
    #[clap(short, long)]
    censor: bool,
    /// Print the execution time for each problem (warning: not amortised).
    #[clap(short, long)]
    time: bool,
    /// Print a summary of execution times as fraction and percentage.
    #[clap(short, long)]
    summary: bool, // todo: probably show this by default
}

// define int type to make everyone's life easier
pub type Int = i64;

pub struct Problem {
    // number: u32, // don't encode number, just use index in static array
    title: &'static str,
    function: fn() -> Int
}

static PROBLEMS: Lazy<Vec<Problem>> = Lazy::new(|| {
    vec![
        Problem {
            title: "1. sum of multiples of 3 or 5 under 1000:",
            function: problem_1
        },
        Problem {
            title: "2. sum of even values of fibonacci sequence under 4 million:",
            function: problem_2
        },
        Problem {
            title: "3. largest prime factor of 600851475143:",
            function: problem_3
        },
        Problem {
            title: "4. largest palindrome number that is product of two 3 digit numbers:",
            function: problem_4
        },
        Problem {
            title: "5. smallest positive number divisible by all numbers from 1 to 20:",
            function: problem_5
        },
        Problem {
            title: "6. difference between sum of squares and square of sum of numbers from 1 to 100:",
            function: problem_6
        },
        Problem {
            title: "7. 10001st prime:",
            function: problem_7
        },
        Problem {
            title: "8. product of 13 adjacent digits in the 1000 digit number with the greatest product:",
            function: problem_8
        },
        Problem {
            title: "9. product of pythagorean triple whose sum is 1000:",
            function: problem_9
        },
        Problem {
            title: "10. sum of all primes under 2 million:",
            function: problem_10
        },
        Problem {
            title: "11. greatest product of 4 numbers in each direction of 20 x 20 array:",
            function: problem_11
        },
        Problem {
            title: "12. first triangle number with over 500 divisors:",
            function: problem_12
        },
        Problem {
            title: "13. first 10 digits of sum of 100 50 digit numbers:",
            function: problem_13
        },
        Problem {
            title: "14. starting number of longest collatz sequence under one million:",
            function: problem_14
        },
        Problem {
            title: "15. lattice paths in 20x20 grid:",
            function: problem_15
        },
        Problem {
            title: "16. sum of digits in 2^1000:",
            function: problem_16
        },
        Problem {
            title: "17. number of characters in 1-1000 (inclusive) written as words:",
            function: problem_17
        },
        Problem {
            title: "18. maximum top to bottom path in 15 row triangle:",
            function: problem_18
        },
        Problem {
            title: "19. number of sundays in 20th century:",
            function: problem_19
        },
        Problem {
            title: "20. sum of digits in the number 100!:",
            function: problem_20
        },
        Problem {
            title: "21. sum of amicable numbers under 10 000:",
            function: problem_21
        },
        Problem {
            title: "22. total of name scores for names in 'p022_names.txt':",
            function: problem_22
        },
        Problem {
            title: "23. sum of positive integers that can't be expressed as sum of two abundant numbers:",
            function: problem_23
        },
        Problem {
            title: "24. millionth lexicograghic permutation of 0-9:",
            function: problem_24
        },
        Problem {
            title: "25. index of first term in fibonacci sequence to contain 1000 digits:",
            function: problem_25
        },
        Problem {
            title: "26. d < 1000 for which 1/d has longest recurring cycle:",
            function: problem_26
        },
        Problem {
            title: "27. product of coefficients for quadratics of form n^2 + an + b producing most primes:",
            function: problem_27
        },
        Problem {
            title: "28. sum of diagonals of 1001 x 1001 number spiral:",
            function: problem_28
        },
        Problem {
            title: "29. distinct terms in sequence generated by a^b for a, b in range 2-100 inclusive:",
            function: problem_29
        },
        Problem {
            title: "30. sum of numbers that can be written as sum of 5th powers of their digits:",
            function: problem_30
        },
        Problem {
            title: "31. number of ways of making £2 in UK currency:",
            function: problem_31
        },
        Problem {
            title: "32. sum of products whose multiplicand/multiplier/product can be written as 1 through 9 pandigital:",
            function: problem_32
        },
        Problem {
            title: "33. find denominator in lowest terms of product of curious fractions:",
            function: problem_33
        },
        Problem {
            title: "34. sum of numbers equal to sum of factorials of their digits:",
            function: problem_34
        },
        Problem {
            title: "35. number of circular primes under one million:",
            function: problem_35
        },
        Problem {
            title: "36. sum of numbers less than 1000000 that are palindromes in base 10 \\ base 2:",
            function: problem_36
        },
        Problem {
            title: "37. sum of primes that are truncatable in both directions:",
            function: problem_37
        },
    ]
});

impl Problem {
    fn print(&self, p: &Printer, summary: &mut Summary) {
        p.print_problem(self.title, self.function, summary)
    }
}

fn main() {
    // fancy errors
    color_eyre::install().unwrap();

    // initialise
    let args = Args::parse();
    let p = Printer::new(args.time, args.censor);
    let mut s = Summary::default();

    // print heading
    Printer::print_heading("Project Euler Solutions");

    match args.problem {
        0 => {
            // just do all of them
            for problem in PROBLEMS.iter() {
                problem.print(&p, &mut s);
            }
        }
        _ => {
            let index = args.problem - 1;
            let problem = PROBLEMS.get(index).expect("no es problemo");
            problem.print(&p, &mut s);
        },
    };

    // todo: idk if this needs a whole argument
    if args.summary {
        Printer::print_summary(&s);
    }
}
