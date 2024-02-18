// AND, OR, XOR 2

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

const DIV: i128 = 998_244_353;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i128>);
    let n = input.next().unwrap() as usize;
    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(input.next().unwrap());
    }
    let mut sum = 0;
    for idx in 0..n {
        let mut p_sum = 0;
        for jdx in idx..n {
            let mut pp_sum = nums[idx];
            for num in &nums[(idx + 1)..=jdx] {
                pp_sum &= num;
            }
            p_sum += pp_sum;
        }
        sum += p_sum % DIV;
    }
    write!(output, "{} ", sum % DIV).unwrap();
    sum = 0;
    for idx in 0..n {
        let mut p_sum = 0;
        for jdx in idx..n {
            let mut pp_sum = nums[idx];
            for num in &nums[(idx + 1)..=jdx] {
                pp_sum |= num;
            }
            p_sum += pp_sum;
        }
        sum += p_sum % DIV;
    }
    write!(output, "{} ", sum % DIV).unwrap();
    sum = 0;
    for idx in 0..n {
        let mut p_sum = 0;
        for jdx in idx..n {
            let mut pp_sum = nums[idx];
            for num in &nums[(idx + 1)..=jdx] {
                pp_sum ^= num;
            }
            p_sum += pp_sum;
        }
        sum += p_sum % DIV;
    }
    write!(output, "{} ", sum % DIV).unwrap();
    println!("{}", output);
    Ok(())
}
