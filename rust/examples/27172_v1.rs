// 수 나누기 게임

use std::collections::HashSet;
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
    for _ in 0..n {
        nums.push(input.next().unwrap());
    }

    let nums_set: HashSet<usize> = nums.iter().cloned().collect();
    let max = nums.iter().max().unwrap() + 1;
    let mut scores = vec![0; max];
    for num in &nums {
        for jdx in (num * 2..=max).step_by(*num) {
            if nums_set.contains(&jdx) {
                scores[*num] += 1;
                scores[jdx] -= 1;
            }
        }
    }
    for num in &nums {
        write!(output, "{} ", scores[*num]).unwrap();
    }
    println!("{}", output);
    Ok(())
}
