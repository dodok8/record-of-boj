// 양둥이 게임

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

    let mut dp = vec![0.0; n + 1];
    dp[1] = 100.0;
    let mut ans = f64::MIN;
    for _ in 0..m {
        let v = input.next().unwrap();
        let w = input.next().unwrap();
        tree[v].push(w);
    }
    for idx in 1..=n {
        if tree[idx].is_empty() {
            ans = ans.max(dp[idx]);
        } else {
            for &jdx in &tree[idx] {
                dp[jdx] += dp[idx] / (tree[idx].len() as f64);
            }
            dp[idx] = 0.0;
        }
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
