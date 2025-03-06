// 합

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;

    let convert_to_num = |&x: &char| (x as u8 - b'A') as usize;

    let mut non_zero = vec![false; 10];
    let mut sums = vec![0; 10];
    let mut nums = vec![0; 10];

    for _ in 0..n {
        let letters = input.next().unwrap().chars().collect::<Vec<char>>();
        for (idx, &letter) in letters.iter().rev().enumerate() {
            let curr_n = convert_to_num(&letter);
            if idx == letters.len() - 1 {
                non_zero[curr_n] = true;
            }
            sums[curr_n] += 10u128.pow(idx as u32);
        }
    }

    let mut unused_false = false;

    let mut some_unused = false;
    for idx in 0..10 {
        if sums[idx] == 0 {
            some_unused = true;
            break;
        }
    }

    if !some_unused {
        let mut zero_idx = 0;
        let mut min_sum = u128::MAX;

        for idx in 0..10 {
            if !non_zero[idx] && sums[idx] < min_sum {
                min_sum = sums[idx];
                zero_idx = idx;
            }
        }

        sums[zero_idx] = 0;
    }

    let mut sorted_nums: Vec<(u128, usize)> =
        sums.iter().enumerate().map(|(a, &b)| (b, a)).collect();

    sorted_nums.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    for (&(_sum, letter), val) in sorted_nums.iter().zip((0..=9).rev()) {
        nums[letter] = val;
    }

    let mut ans = 0;
    for idx in 0..10 {
        ans += sums[idx] * nums[idx];
    }

    writeln!(output, "{}", ans)?;

    print!("{}", output);
    Ok(())
}
