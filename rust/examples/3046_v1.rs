// R2

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let r1 = input.next().unwrap();
    let s = input.next().unwrap();
    writeln!(output, "{}", s * 2 - r1).unwrap();
    print!("{}", output);
    Ok(())
}
