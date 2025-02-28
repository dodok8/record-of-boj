// 연속합

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let nums = input.take(n).collect::<Vec<i32>>();

    let mut dp = vec![0; n]; // dp[n]은 n이 마지막으로 포함된 최대 연속합
    dp[0] = nums[0];

    let mut ans = dp[0];
    for idx in 1..n {
        dp[idx] = nums[idx].max(dp[idx - 1] + nums[idx]);
        if ans < dp[idx] {
            ans = dp[idx]
        }
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
