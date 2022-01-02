use ansi_term::Colour;
use ansi_term::Style;
use std::fmt::Display;

pub fn print_heading(s: &str) {
    let h_style: ansi_term::Style = Style::new().bold().fg(Colour::Blue);
    println!("{}", h_style.paint(s));
}

pub fn print_problem<T: Display>(q: &str, a: T) {
    let style: ansi_term::Style = Style::new().bold().fg(Colour::Purple);
    println!("{} {}", q, style.paint(format!("{}", a)));
}
