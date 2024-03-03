// 갈래의 색종이 자르기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = (input.next().unwrap() * 2) as f64;
    let sqrt_n = n.sqrt() as usize;
    writeln!(output, "{}", sqrt_n * 4).unwrap();
    print!("{}", output);
    Ok(())
}
