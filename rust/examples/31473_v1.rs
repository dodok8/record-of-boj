// 하늘과 핑크

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut sum_a = 0;
    for _ in 0..n {
        sum_a += input.next().unwrap();
    }
    let mut sum_b = 0;
    for _ in 0..n {
        sum_b += input.next().unwrap();
    }
    writeln!(output, "{} {}", sum_b, sum_a).unwrap();
    print!("{}", output);
    Ok(())
}
