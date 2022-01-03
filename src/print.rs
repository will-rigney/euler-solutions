use ansi_term::Colour;
use ansi_term::Style;
use std::fmt::Display;

pub fn print_heading(s: &str) {
    let h_style = Style::new().bold().fg(Colour::Blue);
    println!("{}", h_style.paint(s));
}

pub fn print_problem<T: Display>(q: &str, a: T) {
    let style = Style::new().bold().fg(Colour::Purple);
    println!("{} {}", q, style.paint(format!("{}", a)));
}

pub fn print_missing<T: Display>(s: T) {
    let style = Style::new().bold().fg(Colour::Red);
    println!("{}", style.paint(format!("{}", s)));
}
