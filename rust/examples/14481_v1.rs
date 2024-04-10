// 물통 문제

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn gcd(a: &usize, b: &usize) -> usize {
    if b > a {
        gcd(b, a)
    } else if a % b == 0 {
        *b
    } else {
        gcd(&(a % b), b)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    for _ in 0..n {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        let target = input.next().unwrap();

        if target > a && target > b {
            writeln!(output, "NO").unwrap();
        } else if target % gcd(&a, &b) == 0 {
            writeln!(output, "YES").unwrap();
        } else {
            writeln!(output, "NO").unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
