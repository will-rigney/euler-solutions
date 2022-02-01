# Project Euler Solutions

Solutions to problems available on [Project Euler](https://projecteuler.net),
implemented in the one true meme language [Rust](https://rust-lang.org/).

Contains code for solutions but not the actual solutions, trying to avoid
spoilers for anyone too lazy to at least run the code themselves.

## Profile

![alt text](https://projecteuler.net/profile/mry666.png)

## Usage

Build & run the program using `cargo`.

The program uses [Clap](https://github.com/clap-rs/clap) to parse some argument
flags:

- `-t` will include an ultra basic benchmark, showing the execution time for
  each problem-solving function.
- `-c` will censor the results, to show the execution time without giving
  anything away.
- `-p <number>` will execute & print output for only problem \<number\>.

## Results

Output from running a release build of the program using `-ct` is checked
into the repo at [output.txt](output.txt).

## CI

I have the program running with a GitHub action, at some point would love to
automatically update the published (censored) output. I'm a little scared of
infinity automatic commits though, so maybe l8r.
