// 사회망 서비스(SNS)

use std::cmp::min;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn update_dp(
    dp: &mut Vec<Vec<usize>>,
    tree: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    idx: usize,
) {
    visited[idx] = true;
    dp[idx][1] = 1;
    for &child in &tree[idx] {
        if !visited[child] {
            update_dp(dp, tree, visited, child);
            dp[idx][0] += dp[child][1];
            dp[idx][1] += min(dp[child][1], dp[child][0]);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    let mut visited = vec![false; n + 1];
    let mut dp = vec![vec![0usize; 2]; n + 1];
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

    for _ in 0..(n - 1) {
        let u = input.next().unwrap();
        let v = input.next().unwrap();

        tree[u].push(v);
        tree[v].push(u);
    }
    update_dp(&mut dp, &tree, &mut visited, 1);

    writeln!(output, "{}", min(dp[1][0], dp[1][1]))?;
    print!("{}", output);
    Ok(())
}
