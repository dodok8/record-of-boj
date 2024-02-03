// 노드 사이의 거리

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Weight = usize;
#[derive(Clone)]
struct Edge(Weight, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_v = input.next().unwrap();
    let num_test = input.next().unwrap();
    let mut graph: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];
    let mut cases = vec![];
    for _ in 0..num_v - 1 {
        let v1 = input.next().unwrap();
        let v2 = input.next().unwrap();
        let weight = input.next().unwrap();
        graph[v1].push(Edge(weight, v2));
        graph[v2].push(Edge(weight, v1));
    }
    for _ in 0..num_test {
        let start_v = input.next().unwrap();
        let end_v = input.next().unwrap();
        cases.push((start_v, end_v));
    }

    for (start_v, end_v) in cases {
        let mut travel_stack: VecDeque<(usize, Weight)> = VecDeque::new();
        let mut visited = vec![false; num_v + 1];
        travel_stack.push_back((start_v, 0));
        visited[start_v] = true;
        while !travel_stack.is_empty() {
            let (curr_v, curr_w) = travel_stack.pop_back().unwrap();
            if curr_v == end_v {
                writeln!(output, "{}", curr_w).unwrap();
                break;
            }
            for edge in &graph[curr_v] {
                let next_v = edge.1;
                let next_w = edge.0;
                if !visited[next_v] {
                    visited[next_v] = true;
                    travel_stack.push_back((next_v, curr_w + next_w));
                }
            }
        }
    }
    print!("{}", output);
    Ok(())
}
