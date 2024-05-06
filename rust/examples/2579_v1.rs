// 계단 오르기

use std::cmp;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_dp(n: usize, dp: &mut Vec<usize>, stairs: &Vec<usize>) -> usize {
    if dp[n] == 0 {
        dp[n] = cmp::max(
            get_dp(n - 3, dp, stairs) + stairs[n - 1],
            get_dp(n - 2, dp, stairs),
        ) + stairs[n];
    }
    dp[n]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let stairs = input.take(n).collect::<Vec<usize>>();
    let mut dp = vec![0_usize; n];
    match n {
        1 => writeln!(output, "{}", stairs[0]).unwrap(),
        2 => writeln!(output, "{}", stairs[1]).unwrap(),
        3 => writeln!(
            output,
            "{}",
            cmp::max(stairs[0] + stairs[2], stairs[1] + stairs[2])
        )
        .unwrap(),
        _ => {
            dp[0] = stairs[0];
            dp[1] = stairs[1] + stairs[0];
            dp[2] = cmp::max(stairs[0] + stairs[2], stairs[1] + stairs[2]);
            writeln!(output, "{}", get_dp(n - 1, &mut dp, &stairs)).unwrap();
        }
    }

    print!("{}", output);
    Ok(())
}
