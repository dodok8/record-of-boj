// 숫자의 개수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut n = input.next().unwrap();
    n *= input.next().unwrap();
    n *= input.next().unwrap();
    let binding = n.to_string();
    let digits = binding.chars().map(|d| d.to_digit(10).unwrap());
    let mut counts = vec![0; 10];
    for digit in digits {
        counts[digit as usize] += 1;
    }
    for count in counts {
        writeln!(output, "{}", count).unwrap();
    }
    print!("{}", output);
    Ok(())
}
