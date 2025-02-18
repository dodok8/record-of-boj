// 황소 다마고치

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap() as f64;
        let m = input.next().unwrap() as f64;

        let ans = n.log2().floor() + m + 1.0;
        let ans = ans as usize;
        writeln!(output, "{}", ans)?;
    }

    print!("{}", output);
    Ok(())
}
