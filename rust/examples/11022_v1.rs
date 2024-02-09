// A+B - 8

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u8>);
    let t = input.next().unwrap();
    for idx in 1..=t {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        writeln!(output, "Case #{}: {} + {} = {}", idx, a, b, a + b).unwrap();
    }
    print!("{}", output);
    Ok(())
}
