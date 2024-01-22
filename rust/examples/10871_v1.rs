// X보다 작은 수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let x = input.next().unwrap();
    for _ in 0..n {
        if let Some(val) = input.next() {
            if val < x {
                write!(output, "{} ", val).unwrap();
            }
        }
    }
    println!("{}", output);
    Ok(())
}
