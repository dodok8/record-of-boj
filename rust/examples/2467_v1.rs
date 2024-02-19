// 용액
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap();
    let mut nums = Vec::new();

    for _in in 0..n {
        nums.push(input.next().unwrap());
    }
    nums.sort_unstable();
    let mut start = 0_usize;
    let mut end = (n - 1) as usize;
    let mut closest_sum = i64::MAX;
    let mut first = 0;
    let mut second = 0;
    loop {
        let sum = nums[start] + nums[end];
        match sum.abs().cmp(&closest_sum) {
            std::cmp::Ordering::Less => {
                closest_sum = sum.abs();
                first = nums[start];
                second = nums[end];
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {}
        }
        if sum < 0 {
            start += 1;
        } else {
            end -= 1;
        }
        if start == end {
            break;
        }
    }
    writeln!(output, "{} {}", first, second).unwrap();
    print!("{}", output);
    Ok(())
}
