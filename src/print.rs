use ansi_term::Colour;
use ansi_term::Style;
use std::fmt::Display;
use std::time::Instant;

/// Provides printing functions configurable on initialisation
pub struct Printer {
    should_show_time: bool,
    should_censor: bool,
}

impl Printer {
    /// Creates a new Printer instance
    pub fn new(should_show_time: bool, should_censor: bool) -> Self {
        Self {
            should_show_time,
            should_censor,
        }
    }

    /// Print s using a heading style
    pub fn print_heading(s: &str) {
        let h_style = Style::new().bold().fg(Colour::Blue);
        println!("{}", h_style.paint(s));
    }

    /// Runs a function (for a problem) and prints the result.
    /// If the Printer has been initialised with should_show_time, will also print the execution
    /// time in milliseconds.
    /// If the Printer has been initialised with should_censor, will replace the actual result with
    /// '*' characters when printing.
    /// q is the question string
    /// f is the function to run to provide the problem solution
    pub fn print_problem<T: Display>(&self, q: &str, f: &dyn Fn() -> T) {
        // todo: think this could be better implemented as a proc macro
        // styles for printing
        let style = Style::new().bold().fg(Colour::Purple);
        let time_style = Style::new().bold().fg(Colour::Cyan);

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
        print!("{} {}", q, style.paint(result));

        // print the execution time
        if self.should_show_time {
            let time = if time > 0 {
                format!("{} ms", time)
            } else {
                "<1 ms".to_string()
            };
            print!(", time: {}", time_style.paint(time));
        }

        // newline
        println!();
    }

    /// Print s using a kind of warning style
    /// Meant to be used to notify that a solution for a given problem is not present
    pub fn print_missing<T: Display>(s: T) {
        let style = Style::new().bold().fg(Colour::Red);
        println!("{}", style.paint(format!("{}", s)));
    }
}
