// JOI 공원

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
type Weight = usize;
type Vertex = usize;
type Edge = (Weight, Vertex);

fn trace_parents(end_v: usize, parents: &Vec<Vec<usize>>) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push(end_v);

    while let Some(vertex) = stack.pop() {
        if visited.insert(vertex) {
            for &parent in &parents[vertex] {
                stack.push(parent);
            }
        }
    }

    visited
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_v = input.next().unwrap();
    let num_e = input.next().unwrap();
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v];
    let start_v = input.next().unwrap() - 1;
    let end_v = input.next().unwrap() - 1;

    for _ in 0..num_e {
        let start: Vertex = input.next().unwrap() - 1;
        let end: Vertex = input.next().unwrap() - 1;
        let weight: Weight = input.next().unwrap();
        edges[start].push((weight, end));
        edges[end].push((weight, start));
    }

    let mut weights = vec![usize::MAX; num_v];
    weights[start_v] = 0;
    let mut travel_pq: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    travel_pq.push(std::cmp::Reverse((0, start_v)));
    let mut parents = vec![vec![]; num_v];

    while !travel_pq.is_empty() {
        let std::cmp::Reverse((curr_weight, curr_vertex)) = travel_pq.pop().unwrap();
        if curr_weight > weights[curr_vertex] {
            continue;
        }
        for (weight, next_vertex) in &edges[curr_vertex] {
            let next_weight = weight + curr_weight;
            if weights[*next_vertex] == next_weight {
                parents[*next_vertex].push(curr_vertex);
            }
            if weights[*next_vertex] > next_weight {
                weights[*next_vertex] = next_weight;
                parents[*next_vertex].clear();
                parents[*next_vertex].push(curr_vertex);
                travel_pq.push(std::cmp::Reverse((next_weight, *next_vertex)));
            }
        }
    }
    let mut result = trace_parents(end_v, &parents)
        .into_iter()
        .map(|x| x + 1)
        .collect::<Vec<usize>>();
    result.sort();
    writeln!(output, "{}", result.len()).unwrap();
    for city in result {
        write!(output, "{} ", city).unwrap();
    }

    println!("{}", output);
    Ok(())
}
