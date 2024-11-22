// Patyki

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .collect::<Vec<i32>>();
    input.sort();
    let mut input = input.iter();
    let &a = input.next().unwrap();
    let &b = input.next().unwrap();
    let &c = input.next().unwrap();

    if a == b && b == c {
        writeln!(output, "2")?;
    } else if a.pow(2) + b.pow(2) == c.pow(2) {
        writeln!(output, "1")?;
    } else {
        writeln!(output, "0")?;
    }
    print!("{}", output);
    Ok(())
}
