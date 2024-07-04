// Celebrity jeopardy

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.split_ascii_whitespace();
    for equation in input {
        writeln!(output, "{}", equation).unwrap();
    }
    print!("{}", output);
    Ok(())
}
