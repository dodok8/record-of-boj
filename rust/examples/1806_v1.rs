// 부분합

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let s = input.next().unwrap();
    let mut nums = Vec::new();

    for _ in 0..n {
        nums.push(input.next().unwrap());
    }

    let mut result = usize::MAX;

    let mut start = 0_usize;
    let mut end = 0_usize;
    let mut curr = nums[0];

    while end < nums.len() {
        if curr >= s {
            if end - start < result {
                result = end - start + 1;
            }
            curr -= nums[start];
            start += 1;
        } else {
            end += 1;
            if end == n {
                break;
            }
            curr += nums[end];
        }
    }

    if result == usize::MAX {
        result = 0
    }
    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
