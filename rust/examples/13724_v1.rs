// 수열

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn query(nums: &[i64], start: usize, end: usize, delta: i64, n: usize) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut deq: VecDeque<i64> = VecDeque::new();

    let start = start - 1;
    let end = end - 1;

    deq.extend(nums[start..=end].iter().map(|x| x + delta));
    let mut idx = 0;
    while idx < n {
        if start <= idx && idx <= end {
            idx += 1; // 무한 루프 방지
            continue;
        }
        if deq.is_empty() || *deq.front().unwrap() > nums[idx] {
            result.push(nums[idx]);
            idx += 1;
        } else {
            result.push(deq.pop_front().unwrap());
        }
    }

    result.extend(deq);
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
    }

    for num in nums {
        write!(output, "{} ", num)?;
    }

    println!("{}", output);
    Ok(())
}
