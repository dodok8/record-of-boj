// 트리와 쿼리

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn dfs(
    curr_v: usize,
    graph: &Vec<Vec<usize>>,
    queries: &mut Vec<usize>,
    visited: &mut Vec<bool>,
) -> usize {
    visited[curr_v] = true;
    for &next_v in &graph[curr_v] {
        if !visited[next_v] {
            queries[curr_v] += dfs(next_v, graph, queries, visited);
        }
    }
    queries[curr_v] += 1;
    queries[curr_v]
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let num_v = input.next().unwrap();
    let root_idx = input.next().unwrap();
    let num_q = input.next().unwrap();

    let mut graph = vec![vec![]; num_v + 1];
    let mut queries = vec![0; num_v + 1];

    for _ in 0..(num_v - 1) {
        let v1 = input.next().unwrap();
        let v2 = input.next().unwrap();

        graph[v1].push(v2);
        graph[v2].push(v1);
    }
    let mut visited = vec![false; num_v + 1];
    dfs(root_idx, &graph, &mut queries, &mut visited);
    for _ in 0..num_q {
        let target_idx = input.next().unwrap();
        writeln!(output, "{}", queries[target_idx]).unwrap();
    }
    print!("{}", output);
    Ok(())
}
