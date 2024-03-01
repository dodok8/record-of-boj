// 초콜릿과 11과 팰린드롬

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let mut temp = String::new();
        if n == 1 {
            writeln!(temp, "0")?;
        } else if n % 2 == 0 {
            for __ in 0..n {
                write!(temp, "1")?;
            }
            writeln!(temp)?;
        } else if ((n - 1) / 2) % 2 == 0 {
            for __ in 0..(n / 2) {
                write!(temp, "1")?;
            }
            write!(temp, "0")?;
            for __ in 0..(n / 2) {
                write!(temp, "1")?;
            }
            writeln!(temp)?;
        } else {
            for __ in 0..(n / 2) {
                write!(temp, "1")?;
            }
            write!(temp, "2")?;
            for __ in 0..(n / 2) {
                write!(temp, "1")?;
            }
            writeln!(temp)?;
        }
        print!("{}", temp);
    }
    Ok(())
}
