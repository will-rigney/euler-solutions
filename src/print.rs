use colored::*;

use std::{fmt::Display, time::Instant};
use crate::Int;

/// Provides printing functions configurable on initialisation
pub struct Printer {
    should_show_time: bool,
    should_censor: bool,
}

#[derive(Default)]
pub struct Summary {
    times: Vec<u64>
}

impl Printer {
    /// Creates a new Printer instance with settings
    pub fn new(should_show_time: bool, should_censor: bool) -> Self {
        Self {
            should_show_time,
            should_censor,
        }
    }

    /// Print `s` using a heading style
    pub fn print_heading(s: &str) {
        println!("{}", s.blue().bold());
    }

    /// Runs a function (for a problem) and prints the result.
    /// If the Printer has been initialised with should_show_time, will also print the execution
    /// time in milliseconds.
    /// If the Printer has been initialised with should_censor, will replace the actual result with
    /// '*' characters when printing.
    /// q is the question string
    /// f is the function to run to provide the problem solution
    pub fn print_problem(&self, q: &str, f: fn() -> Int, summary: &mut Summary) {
        // time & run the problem function
        let start = Instant::now();
        let result = f();
        let time = start.elapsed().as_millis();

        let mut result = format!("{}", result);

        // censor
        if self.should_censor {
            result = "*".repeat(result.len());
        }
        // print the question & answer
        print!("{} {}", q, result.purple().bold());

        // "log" the execution time in summary
        summary.times.push(time as u64);

        // print the execution time
        if self.should_show_time {
            let time = if time > 0 {
                format!("{} ms", time)
            } else {
                "<1 ms".to_string()
            };
            print!(", time: {}", time.red().bold());
        }

        // newline
        println!();
    }

    /// Print s using a kind of warning style
    /// Meant to be used to notify that a solution for a given problem is not present
    pub fn print_missing<T: Display>(s: T) {
        println!("{}", format!("{}", s).red().bold());
    }

    pub fn print_summary(summary: &Summary) {
        let total = summary.times.len();
        let t1ms = summary.times.iter().filter(|t| **t < 1).count();
        let t10ms = summary.times.iter().filter(|t| **t < 10).count();
        let t100ms = summary.times.iter().filter(|t| **t < 100).count();
        println!("{}", "Summary".blue());
        println!("<1ms: {}/{} ({}%)", t1ms, total, t1ms * 100 / total);
        println!("<10ms: {}/{} ({}%)", t10ms, total, t10ms * 100 / total);
        println!("<100ms: {}/{} ({}%)", t100ms, total, t100ms * 100 / total);
    }
}
