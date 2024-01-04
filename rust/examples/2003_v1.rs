//수들의 합 2

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let mut sums = vec![0; n + 1];
    for idx in 1..(n + 1) {
        sums[idx] = sums[idx - 1] + input.next().unwrap();
    }

    let mut count = 0;
    let mut idx = 1;
    let mut jdx = 1;

    while jdx <= n && idx <= jdx {
        match (sums[jdx] - sums[idx - 1]).cmp(&m) {
            std::cmp::Ordering::Equal => {
                count += 1;
                jdx += 1;
            }
            std::cmp::Ordering::Less => {
                jdx += 1;
            }
            std::cmp::Ordering::Greater => {
                idx += 1;
            }
        }
    }
    writeln!(output, "{}", count).unwrap();
    print!("{}", output);
    Ok(())
}
