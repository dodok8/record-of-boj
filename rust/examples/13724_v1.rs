// 수열

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn query(nums: &[i64], start: usize, end: usize, delta: i64, n: usize) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();

    let start = start - 1;
    let end = end - 1;

    let mut d_idx = start;
    let mut o_idx = 0;

    while o_idx < n {
        if d_idx > end {
            result.push(nums[o_idx]);
            o_idx += 1;
        } else {
            if start <= o_idx && o_idx <= end {
                o_idx += 1;
                continue;
            }
            if nums[d_idx] + delta < nums[o_idx] {
                result.push(nums[d_idx] + delta);
                d_idx += 1;
            } else {
                result.push(nums[o_idx]);
                o_idx += 1;
            }
        }
    }

    while o_idx == n && d_idx <= end {
        result.push(nums[d_idx] + delta);
        d_idx += 1;
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap() as usize;
    let num_q = input.next().unwrap() as usize;

    let mut nums = input.by_ref().take(n).collect::<Vec<i64>>();

    for _ in 0..num_q {
        nums = query(
            &nums,
            input.next().unwrap() as usize,
            input.next().unwrap() as usize,
            input.next().unwrap(),
            n,
        );
        for num in nums.iter() {
            write!(output, "{} ", num)?;
        }
        writeln!(output)?;
    }

    println!("{}", output);
    Ok(())
}
