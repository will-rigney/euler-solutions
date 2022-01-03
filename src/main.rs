mod print;
mod problems_11_20;
mod problems_1_10;
use print::*;
use problems_11_20::*;
use problems_1_10::*;

fn main() {
    print_heading("Euler Project Solutions");

    print_problem("1. sum of multiples of 3 or 5 under 1000:", problem_1());
    print_problem(
        "2. sum of even values of fibonacci sequence under 4 million:",
        problem_2(),
    );
    print_problem("3. largest prime factor of 600851475143:", problem_3());
    print_problem(
        "4. largest palindrome number that is product of two 3 digit numbers:",
        problem_4(),
    );
    print_problem(
        "5. smallest positive number divisible by all numbers from 1 to 20:",
        problem_5(),
    );
    print_problem(
        "6. difference between sum of squares and square of sum of numbers from 1 to 100:",
        problem_6(),
    );
    print_problem("7. 10001st prime:", problem_7());
    print_problem(
        "8. product of 13 adjacent digits in the 1000 digit number with the greatest product:",
        problem_8(),
    );
    print_problem(
        "9. product of pythagorean triple whose sum is 1000:",
        problem_9(),
    );
    print_problem("10. sum of all primes under 2 million:", problem_10());

    print_problem(
        "11. greatest product of 4 numbers in each direction of 20 x 20 array:",
        problem_11(),
    );

    print_problem(
        "12. first triangle number with over 500 divisors:",
        problem_12(),
    );

    print_problem(
        "14. starting number of longest collatz sequence under one million:",
        problem_14(),
    );

    print_problem("15. lattice paths in 20x20 grid:", problem_15());

    print_problem("16. sum of digits in 2^1000:", problem_16());
}
