use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    for num in [1, 1, 2, 2, 2, 8] {
        write!(output, "{} ", num - input.next().unwrap()).unwrap();
    }

    println!("{}", output);
    Ok(())
}
