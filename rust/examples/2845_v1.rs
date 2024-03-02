// 파티가 끝나고 난 뒤

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mem = input.next().unwrap() * input.next().unwrap();
    for _ in 0..5 {
        write!(output, "{} ", input.next().unwrap() - mem).unwrap();
    }
    println!("{}", output);
    Ok(())
}
