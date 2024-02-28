// 주식

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let q = input.next().unwrap();
    for __ in 0..q {
        let n = input.next().unwrap();
        let mut stocks = Vec::new();
        for _ in 0..n {
            stocks.push(input.next().unwrap());
        }

        let mut sum = 0;
        let mut curr_max = stocks[n - 1];

        for idx in (0..n - 1).rev() {
            if stocks[idx] < curr_max {
                sum += curr_max - stocks[idx];
            } else {
                curr_max = stocks[idx];
            }
        }
        writeln!(output, "{}", sum).unwrap();
    }
    print!("{}", output);
    Ok(())
}
