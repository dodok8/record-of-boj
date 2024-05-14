// 파도반 수열

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_dp(n: usize, dp: &mut Vec<usize>) -> usize {
    if dp[n] == 0 {
        dp[n] = get_dp(n - 2, dp) + get_dp(n - 3, dp);
    }
    dp[n]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut dp = vec![0_usize; 100];
    dp[1] = 1;
    dp[2] = 1;
    dp[3] = 1;
    dp[4] = 2;
    dp[5] = 2;
    let _n = input.next().unwrap();
    for num in input {
        writeln!(output, "{}", get_dp(num, &mut dp)).unwrap();
    }
    print!("{}", output);
    Ok(())
}
