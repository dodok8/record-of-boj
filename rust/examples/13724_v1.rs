// 수열

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn query(nums: &[i64], start: usize, end: usize, delta: i64, n: usize) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut d_deq: VecDeque<i64> = VecDeque::new();

    let start = start - 1;
    let end = end - 1;

    d_deq.extend(nums[start..=end].iter().map(|x| x + delta));
    let mut o_deq: VecDeque<i64> = VecDeque::new();
    for idx in 0..n {
        if start <= idx && idx <= end {
            continue;
        }
        o_deq.push_back(nums[idx]);
    }

    while !d_deq.is_empty() && !o_deq.is_empty() {
        let d = d_deq.front().unwrap();
        let o = o_deq.front().unwrap();
        if d > o {
            result.push(o_deq.pop_front().unwrap());
        } else {
            result.push(d_deq.pop_front().unwrap());
        }
    }

    result.extend(d_deq);
    result.extend(o_deq);
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
