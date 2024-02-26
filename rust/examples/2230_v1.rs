// 수 고르기
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let mut nums = Vec::new();

    for _in in 0..n {
        nums.push(input.next().unwrap());
    }
    nums.sort_unstable();

    let mut start = 0_usize;
    let mut end = 1_usize;
    let n = n as usize;
    let mut closest_diff = i64::MAX;
    while start < n && end < n {
        let diff = nums[end] - nums[start];
        match diff.cmp(&m) {
            std::cmp::Ordering::Greater => {
                closest_diff = std::cmp::min(closest_diff, diff);
                start += 1;
            }
            std::cmp::Ordering::Equal => {
                closest_diff = diff;
                break;
            }
            std::cmp::Ordering::Less => {
                end += 1;
            }
        }
    }
    writeln!(output, "{}", closest_diff).unwrap();
    print!("{}", output);
    Ok(())
}
