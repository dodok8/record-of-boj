// 이상한 기호

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i128>);
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    writeln!(output, "{}", a.pow(2) - b.pow(2)).unwrap();
    print!("{}", output);
    Ok(())
}
