// 수열

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn query(nums: &mut [i64], start: usize, end: usize, delta: i64, n: usize, temp: &mut [i64]) {
    let start = start - 1;
    let end = end - 1;

    let mut d_idx = start;
    let mut o_idx = 0;
    let mut t_idx = 0;

    for idx in start..=end {
        nums[idx] += delta;
    }
    // println!("{:?}", nums);
    // println!("{:?}", temp);

    while t_idx < n {
        // println!(
        //     "o_dix: {} nums[o_idx]: {} d_idx: {} nums[d_idx]: {} t_idx: {} start: {} end: {}",
        //     o_idx, nums[o_idx], d_idx, nums[d_idx], t_idx, start, end
        // );
        if start <= o_idx && o_idx <= end {
            o_idx += 1;
            // println!("여기야3");
            continue;
        } else if d_idx > end {
            // println!("여기야1");
            temp[t_idx] = nums[o_idx];
            o_idx += 1;
            t_idx += 1;
        } else if o_idx == n && d_idx <= end {
            // println!("여기야2");
            temp[t_idx] = nums[d_idx];
            d_idx += 1;
            t_idx += 1;
        } else if nums[d_idx] < nums[o_idx] {
            temp[t_idx] = nums[d_idx];
            // println!("여기야4");
            d_idx += 1;
            t_idx += 1;
        } else {
            // println!("여기야5");
            temp[t_idx] = nums[o_idx];
            o_idx += 1;
            t_idx += 1;
        }
        // println!("nums : {:?}", nums);
        // println!("temp : {:?}", temp);
    }

    while o_idx == n && d_idx <= end {
        temp[t_idx] = nums[d_idx] + delta;
        d_idx += 1;
        t_idx += 1;
    }

    for (tdx, &t) in temp.iter().enumerate() {
        nums[tdx] = t;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap() as usize;
    let num_q = input.next().unwrap() as usize;

    let mut nums = input.by_ref().take(n).collect::<Vec<i64>>();
    let mut temp = nums.clone();

    for _ in 0..num_q {
        query(
            &mut nums,
            input.next().unwrap() as usize,
            input.next().unwrap() as usize,
            input.next().unwrap(),
            n,
            &mut temp,
        );
        for num in nums.iter() {
            write!(output, "{} ", num)?;
        }
        writeln!(output)?;
    }

    println!("{}", output);
    Ok(())
}
