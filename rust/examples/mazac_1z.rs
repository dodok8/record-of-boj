// Alea lacta Est

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let dices: i32 = (input.sum::<i32>() + 3) % 4;
    if dices == 0 {
        writeln!(output, "4")?;
    } else {
        writeln!(output, "{}", dices)?;
    }
    print!("{}", output);
    Ok(())
}
