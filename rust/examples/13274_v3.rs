// 수열

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap() as usize;
    let num_q = input.next().unwrap() as usize;

    let mut nums = input.by_ref().take(n).collect::<Vec<i64>>();
    nums.sort_unstable();

    for _ in 0..num_q {
        let start = input.next().unwrap() as usize - 1;
        let end = input.next().unwrap() as usize - 1;
        let delta = input.next().unwrap();

        for idx in start..=end {
            nums[idx] += delta
        }
        nums.sort_unstable();
    }
    for num in nums.iter() {
        write!(output, "{} ", num)?;
    }
    writeln!(output)?;
    println!("{}", output);
    Ok(())
}
