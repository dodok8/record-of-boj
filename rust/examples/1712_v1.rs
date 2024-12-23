// 손익 분기점

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();

    if b >= c {
        writeln!(output, "-1")?;
    } else {
        let result = (a as f64 / (c - b) as f64) as usize;
        writeln!(output, "{}", result + 1)?;
    }
    print!("{}", output);
    Ok(())
}
