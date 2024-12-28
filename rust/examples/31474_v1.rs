// 양갈래 짝 맞추기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u128>);
    let n = input.next().unwrap();
    let mut factorial: Vec<u128> = Vec::new();
    factorial.push(1);
    for idx in 1..(n + 1) {
        factorial.push(factorial[idx as usize - 1] * idx);
    }

    let n = n as usize;
    let div: u128 = 2_u128.pow(n as u32 / 2);
    writeln!(output, "{}", factorial[n] / factorial[n / 2] / div)?;
    print!("{}", output);
    Ok(())
}
