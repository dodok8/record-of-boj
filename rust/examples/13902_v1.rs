// 개업 2

use std::cmp::min;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_dp(n: i64, dp: &mut Vec<usize>, woks: &Vec<usize>) -> usize {
    if n < 0 {
        return usize::MAX;
    }
    let n = n as usize;

    if dp[n] == 0 && n != 0 {
        dp[n] = {
            let mut result = usize::MAX;
            for &wok in woks {
                result = min(
                    result,
                    get_dp(n as i64 - wok as i64, dp, woks).saturating_add(1),
                );
            }
            result
        };
    }

    dp[n]
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let num_f = input.next().unwrap();
    let num_w = input.next().unwrap();

    let mut woks = input.take(num_w).collect::<Vec<usize>>();

    for idx in 0..num_w {
        for jdx in 0..idx {
            woks.push(woks[idx] + woks[jdx]);
        }
    }

    let mut dp = vec![0; num_f + 1];
    dp[0] = 0;
    for &wok in &woks {
        dp[wok] = 1;
    }
    writeln!(output, "{}", get_dp(num_f as i64, &mut dp, &woks)).unwrap();
    print!("{}", output);
    Ok(())
}
