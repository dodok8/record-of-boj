// 두수의 합
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    for _ in 0..input.next().unwrap() as usize {
        let n = input.next().unwrap();
        let mut nums = Vec::new();
        let k = input.next().unwrap();

        for _in in 0..n {
            nums.push(input.next().unwrap());
        }
        nums.sort_unstable();
        let mut count = 0;
        let mut start = 0_usize;
        let mut end = (n - 1) as usize;
        let mut closest_sum = 0_i64;
        loop {
            let sum = nums[start] + nums[end];
            match i64::abs(sum - k).cmp(&i64::abs(closest_sum - k)) {
                std::cmp::Ordering::Less => {
                    closest_sum = sum;
                    count = 1;
                }
                std::cmp::Ordering::Equal => {
                    count += 1;
                }
                _ => {}
            }
            if sum < k {
                start += 1;
            } else {
                end -= 1;
            }
            if start == end {
                break;
            }
        }
        writeln!(output, "{}", count).unwrap();
    }
    print!("{}", output);
    Ok(())
}
