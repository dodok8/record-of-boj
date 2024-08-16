// 팬덤이 넘쳐 흘러

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    let mut starts = Vec::new();
    let mut ends = Vec::new();

    for _ in 0..n {
        starts.push(input.next().unwrap());
        ends.push(input.next().unwrap());
    }
    let diff = starts
        .iter()
        .max()
        .unwrap()
        .saturating_sub(*ends.iter().min().unwrap());
    writeln!(output, "{}", diff)?;
    print!("{}", output);
    Ok(())
}
