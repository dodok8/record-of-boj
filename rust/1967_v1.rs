// 트리의 지름

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
    let mut graph: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];

    for _ in 0..num_v - 1 {
        let v1 = input.next().unwrap();
        let v2 = input.next().unwrap();
        let weight = input.next().unwrap();
        graph[v1].push(Edge(weight, v2));
        graph[v2].push(Edge(weight, v1));
    }

    let mut is_leaf = vec![false; num_v + 1];
    let mut leaf = Vec::new();
    for (idx, edge) in graph.iter().enumerate().skip(1).take(num_v) {
        if edge.len() == 1 {
            is_leaf[idx] = true;
            leaf.push(idx);
        }
    }
    let mut curr_max = 0;
    for start_v in leaf {
        let mut travel_stack: VecDeque<(usize, Weight)> = VecDeque::new();
        let mut visited = vec![false; num_v + 1];
        travel_stack.push_back((start_v, 0));
        visited[start_v] = true;
        while !travel_stack.is_empty() {
            let (curr_v, curr_w) = travel_stack.pop_back().unwrap();
            for edge in &graph[curr_v] {
                let next_v = edge.1;
                let next_w = edge.0;
                if !visited[next_v] {
                    visited[next_v] = true;
                    travel_stack.push_back((next_v, curr_w + next_w));
                }
            }
            if is_leaf[curr_v] && curr_max < curr_w {
                curr_max = curr_w;
            }
        }
    }
    writeln!(output, "{}", curr_max).unwrap();
    print!("{}", output);
    Ok(())
}
