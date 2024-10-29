// 피보나치 수 7

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn fibo(n: usize, dp: &mut [i64]) -> i64 {
    if dp[n] == -1 {
        dp[n] = (fibo(n - 1, dp) + fibo(n - 2, dp)) % 1_000_000_007;
    }

    dp[n]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut dp = vec![-1; 1_000_001_usize];
    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;

    let n = input.next().unwrap();
    writeln!(output, "{}", fibo(n, &mut dp))?;
    print!("{}", output);
    Ok(())
}
