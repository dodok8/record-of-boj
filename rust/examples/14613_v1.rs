// 너의 티어는?

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

const D_P: usize = 50;

fn update(idx: usize, jdx: usize, w: f64, l: f64, d: f64, dp: &mut [Vec<f64>]) {
    if dp[idx][jdx] > 0.0 && idx < 20 {
        if jdx + D_P < 3500 {
            dp[idx + 1][jdx + D_P] += dp[idx][jdx] * w;
        }
        if jdx >= D_P {
            dp[idx + 1][jdx - D_P] += dp[idx][jdx] * l;
        }

        dp[idx + 1][jdx] += dp[idx][jdx] * d;
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let w = input.next().unwrap();
    let l = input.next().unwrap();
    let d = input.next().unwrap();

    let mut dp = vec![vec![0_f64; 3500]; 21];

    dp[0][2000] = 1_f64;
    for idx in 0..21 {
        for jdx in 1000..3500 {
            update(idx, jdx, w, l, d, &mut dp);
        }
    }

    for start in [1000, 1500, 2000, 2500, 3000] {
        writeln!(output, "{:.8}", {
            let mut result = 0.0;
            for idx in start..(start + 500) {
                result += dp[20][idx];
            }
            result
        })
        .unwrap();
    }

    print!("{}", output);
    Ok(())
}
