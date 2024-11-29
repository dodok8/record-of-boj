// 알고리즘 수업 - 선택 정렬

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn selection_sort(nums: &mut [usize], k: usize) -> Option<(usize, usize)> {
    let mut swap_count = 0;
    let mut k_swap = None;
    
    for idx in (1..nums.len()).rev() {
        let max_idx = (0..=idx).max_by_key(|&i| nums[i]).unwrap();
        if idx != max_idx {
            swap_count += 1;
            if swap_count == k {
                k_swap = Some((nums[max_idx].min(nums[idx]), nums[max_idx].max(nums[idx])));
            }
            nums.swap(max_idx, idx);
        }
    }
    k_swap
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let k = input.next().unwrap();
    let mut nums = input.take(n).collect::<Vec<usize>>();

    match selection_sort(&mut nums, k) {
        Some((a, b)) => writeln!(output, "{} {}", a, b)?,
        None => writeln!(output, "-1")?,
    }

    print!("{}", output);
    Ok(())
}