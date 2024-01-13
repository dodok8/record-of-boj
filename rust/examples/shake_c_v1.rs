// 또 수열 문제야

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut nums = Vec::new();
    nums.push(1);
    nums.push(4);
    for idx in 2..n {
        nums.push(nums[idx - 2] + nums[idx - 1] + nums[idx - 2] * nums[idx - 1]);
    }
    for num in nums {
        write!(output, "{} ", num).unwrap();
    }
    println!("{}", output);
    Ok(())
}
