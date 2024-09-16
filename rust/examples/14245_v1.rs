// XOR

use std::error::Error;
use std::fmt::{Debug, Display, Write};
use std::io::{stdin, Read};
use std::mem::size_of_val;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut data = input
        .by_ref()
        .take(n)
        .map(|d| (d, Vec::new()))
        .collect::<Vec<(usize, Vec<usize>)>>();
    let m = input.next().unwrap();

    for _ in 0..m {
        if input.next().unwrap() == 1 {
            let s = input.next().unwrap();
            let b = input.next().unwrap();
            let n = input.next().unwrap();
            for idx in s..=b {
                data[idx].1.push(n);
            }
        } else {
            let idx = input.next().unwrap();
            let mut result = data[idx].0;
            for val in &data[idx].1 {
                result ^= val;
            }
            data[idx].0 = result;
            data[idx].1.clear();
            writeln!(output, "{}", data[idx].0)?;
        }
    }
    print!("{}", output);
    Ok(())
}
