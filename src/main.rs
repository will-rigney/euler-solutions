mod print;
mod problems_11_20;
mod problems_1_10;
use clap::Parser;
use print::*;
use problems_11_20::*;
use problems_1_10::*;

#[derive(Parser, Debug)]
#[clap(
    about = "Solutions to problems at https://projecteuler.net/ implemented in Rust.",
    version = "0.0.0",
    author = "Will Rigney"
)]

struct Args {
    /// Specify the problem to solve; defaults to 0 which means all problems.
    #[clap(short, long, default_value_t = 0)]
    problem: u64,
}

fn main() {
    let args = Args::parse();

    let n_problems = 20;

    // problems to solve
    let problems = match args.problem {
        0 => {
            print_heading("Euler Project Solutions");
            (1..=n_problems).collect()
        }
        _ => vec![args.problem],
    };

    for problem in problems {
        match problem {
        1 => print_problem("1. sum of multiples of 3 or 5 under 1000:", problem_1()),
        2 => print_problem(
            "2. sum of even values of fibonacci sequence under 4 million:",
            problem_2(),
        ),
        3 => print_problem("3. largest prime factor of 600851475143:", problem_3()),
        4 => print_problem(
            "4. largest palindrome number that is product of two 3 digit numbers:",
            problem_4(),
        ),
        5 => print_problem(
            "5. smallest positive number divisible by all numbers from 1 to 20:",
            problem_5(),
        ),
        6 => print_problem(
            "6. difference between sum of squares and square of sum of numbers from 1 to 100:",
            problem_6(),
        ),
        7 => print_problem("7. 10001st prime:", problem_7()),
        8 => print_problem(
            "8. product of 13 adjacent digits in the 1000 digit number with the greatest product:",
            problem_8(),
        ),
        9 => print_problem(
            "9. product of pythagorean triple whose sum is 1000:",
            problem_9(),
        ),
        10 => print_problem("10. sum of all primes under 2 million:", problem_10()),
        11 => print_problem(
            "11. greatest product of 4 numbers in each direction of 20 x 20 array:",
            problem_11(),
        ),
        12 => print_problem(
            "12. first triangle number with over 500 divisors:",
            problem_12(),
        ),
        14 => print_problem(
            "14. starting number of longest collatz sequence under one million:",
            problem_14(),
        ),
        15 => print_problem("15. lattice paths in 20x20 grid:", problem_15()),
        16 => print_problem("16. sum of digits in 2^1000:", problem_16()),
        17 => print_problem("17. number of characters in 1-1000 (inclusive) written as words:", problem_17()),
        18 => print_problem("18. maximum top to bottom path in 15 row triangle:", problem_18()),
        19 => print_problem("19. number of sundays in 20th century:", problem_19()),
        20 => print_problem("20. sum of digits in the number 100!:", problem_20()),
        _ => print_missing(format!("no solution for problem {}", problem))
        }
    }
}
