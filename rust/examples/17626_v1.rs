// Four Squares

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_dp(n: usize, dp: &mut Vec<usize>) -> usize {
    if dp[n] == 0 && n != 0 {
        let mut value = usize::MAX;
        let sqrt_n = (n as f64).sqrt() as usize;
        for num in (1..=sqrt_n).rev() {
            let temp = get_dp(n - num.pow(2), dp) + 1;
            if temp == 1 || temp == 2 {
                value = temp;
                break;
            }
            if temp < value {
                value = temp;
            }
        }
        dp[n] = value;
    }
    dp[n]
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut dp = vec![0_usize; n + 1];
    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 3;
    dp[4] = 1;
    dp[5] = 2;

    writeln!(output, "{}", get_dp(n, &mut dp)).unwrap();
    print!("{}", output);
    Ok(())
}
