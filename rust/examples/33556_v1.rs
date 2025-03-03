// Java String Equals

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let a = input.next().unwrap();
    let b = input.next().unwrap();

    if a == "null" {
        writeln!(output, "NullPointerException")?;
        writeln!(output, "NullPointerException")?;
    } else if b == "null" {
        writeln!(output, "false")?;
        writeln!(output, "false")?;
    } else {
        if a == b {
            writeln!(output, "true")?;
        } else {
            writeln!(output, "false")?;
        }

        if a.to_lowercase() == b.to_lowercase() {
            writeln!(output, "true")?;
        } else {
            writeln!(output, "false")?;
        }
    }
    print!("{}", output);
    Ok(())
}
