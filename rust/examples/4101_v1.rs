use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    loop {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());
        if a == 0 && b == 0 {
            break;
        } else if a > b {
            writeln!(output, "Yes").unwrap();
        } else {
            writeln!(output, "No").unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
