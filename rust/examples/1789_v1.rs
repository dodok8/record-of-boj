// 수들의 합

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let s = input.next().unwrap();
    let n = (((1.0 + 8.0 * s).sqrt() - 1.0) / 2.0).floor() as usize;
    writeln!(output, "{}", n).unwrap();
    print!("{}", output);
    Ok(())
}
