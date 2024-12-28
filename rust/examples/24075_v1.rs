// 計算 (Calculation)

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let mut v = [a + b, a - b];
    v.sort_unstable();
    for val in v.iter().rev() {
        writeln!(output, "{}", val)?;
    }
    print!("{}", output);
    Ok(())
}
