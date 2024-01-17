// 스네이크버드

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut length = input.next().unwrap();
    let mut fruits = Vec::new();
    for _ in 0..n {
        fruits.push(input.next().unwrap());
    }
    fruits.sort_unstable();

    for fruit in fruits {
        if length >= fruit {
            length += 1;
        }
    }

    writeln!(output, "{}", length).unwrap();
    print!("{}", output);
    Ok(())
}
