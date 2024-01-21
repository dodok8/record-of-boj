// 카드 게임

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut sum = 0;
    for _ in 0..5 {
        sum += input.next().unwrap();
    }
    writeln!(output, "{}", sum).unwrap();
    print!("{}", output);
    Ok(())
}
