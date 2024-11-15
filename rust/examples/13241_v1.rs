// 최소 공배수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_gcd(a: &usize, b: &usize) -> usize {
    if b > a {
        get_gcd(b, a)
    } else if a % b == 0 {
        *b
    } else {
        get_gcd(&(a % b), b)
    }
}

fn get_lcm(a: &usize, b: &usize) -> usize {
    let gcd = get_gcd(a, b);
    b * a / gcd
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    writeln!(output, "{}", get_lcm(&a, &b))?;
    print!("{}", output);
    Ok(())
}
