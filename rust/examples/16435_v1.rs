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

    let mut count = 0;
    for fruit in fruits {
        if length >= fruit {
            count += 1;
            length += 1;
        }
    }

    writeln!(output, "{}", count).unwrap();
    print!("{}", output);
    Ok(())
}
