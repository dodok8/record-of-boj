// 동가수열 구하기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    
    if n % 2 == 1 {
        let mid = (n + 1) / 2;
        write!(output, "{}", mid)?;
        for i in (mid + 1..=n).rev() {
            write!(output, " {} {}", i, i - mid)?;
        }
    } else {
        let mid = n / 2;
        for i in (mid + 1..=n).rev() {
            write!(output, "{} {} ", i - mid, i)?;
        }
    }

    print!("{}", output);
    Ok(())
}