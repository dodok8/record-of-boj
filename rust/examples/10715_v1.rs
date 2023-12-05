// JOI 공원

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
type Weight = usize;
type Vertex = usize;
type Edge = (Weight, Vertex);

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_v = input.next().unwrap();
    let num_e = input.next().unwrap();
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v];

    let c = input.next().unwrap();
    let mut total_weight = 0;
    for _ in 0..num_e {
        let start: Vertex = input.next().unwrap() - 1;
        let end: Vertex = input.next().unwrap() - 1;
        let weight: Weight = input.next().unwrap();
        edges[start].push((weight, end));
        edges[end].push((weight, start));
        total_weight += weight;
    }

    let mut weights = vec![usize::MAX; num_v];
    let mut repair_cost = total_weight;
    let mut visited = vec![false; num_v];
    weights[0] = 0;
    let mut travel_pq: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    travel_pq.push(std::cmp::Reverse((0, 0)));

    while !travel_pq.is_empty() {
        let std::cmp::Reverse((curr_weight, curr_vertex)) = travel_pq.pop().unwrap();
        if curr_weight > weights[curr_vertex] || visited[curr_vertex] {
            continue;
        }
        visited[curr_vertex] = true;

        for (weight, next_vertex) in &edges[curr_vertex] {
            if visited[*next_vertex] {
                total_weight -= weight;
            }
            // 각 지점을 한번만 방문하는게 보장 되므로 이렇게 total_weight 돌려써도 됨.
        }
        // 최소 수리값 갱신하면 업뎃
        if repair_cost > total_weight + c * curr_weight {
            repair_cost = total_weight + c * curr_weight;
        }
        for (weight, next_vertex) in &edges[curr_vertex] {
            let next_weight = weight + curr_weight;
            if weights[*next_vertex] > next_weight {
                weights[*next_vertex] = next_weight;
                travel_pq.push(std::cmp::Reverse((next_weight, *next_vertex)));
            }
        }
    }

    writeln!(output, "{}", repair_cost).unwrap();
    print!("{}", output);
    Ok(())
}
