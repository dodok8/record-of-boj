// 외판원 순회 2

use std::cmp::min;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn dfs(
    curr_v: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    curr_d: usize,
    curr_w: usize,
    n: usize,
    start: usize,
) -> usize {
    if curr_d == n - 1 {
        curr_w + graph[curr_v][start]
    } else {
        visited[curr_v] = true;
        let mut result = usize::MAX;
        for next_v in 0..n {
            if !visited[next_v] {
                result = min(
                    result,
                    dfs(
                        next_v,
                        graph,
                        visited,
                        curr_d + 1,
                        curr_w + graph[curr_v][next_v],
                        n,
                        start,
                    ),
                );
                visited[next_v] = false;
            }
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut weights = Vec::new();

    for _ in 0..n {
        weights.push(input.by_ref().take(n).collect::<Vec<usize>>());
    }
    let mut min_w = usize::MAX;
    for start in 0..n {
        let mut visited = vec![false; n];
        min_w = min(dfs(start, &weights, &mut visited, 0, 0, n, start), min_w);
    }
    writeln!(output, "{}", min_w).unwrap();
    print!("{}", output);
    Ok(())
}
