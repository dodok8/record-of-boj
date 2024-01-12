// 세제곱의 합

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    write!(
        output,
        "{}\n{}\n{}\n",
        n * (n + 1) / 2,
        usize::pow(n * (n + 1) / 2, 2),
        usize::pow(n * (n + 1) / 2, 2)
    )
    .unwrap();
    print!("{}", output);
    Ok(())
}
