// 파티

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
    let dest = input.next().unwrap();
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];
    let mut reverse_edges: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];

    for _ in 0..num_e {
        let start = input.next().unwrap();
        let end = input.next().unwrap();
        let weight = input.next().unwrap();
        edges[start].push((weight, end));
        reverse_edges[end].push((weight, start));
    }

    let mut distances = vec![usize::MAX; num_v + 1];
    let mut travel_pq: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    distances[dest] = 0;
    travel_pq.push(Reverse((0, dest)));
    while !travel_pq.is_empty() {
        let Reverse((curr_w, curr_v)) = travel_pq.pop().unwrap();

        if curr_w > distances[curr_v] {
            continue;
        }
        for (d_w, next_v) in &edges[curr_v] {
            let next_w = d_w + curr_w;
            if distances[*next_v] > next_w {
                distances[*next_v] = next_w;
                travel_pq.push(Reverse((next_w, *next_v)));
            }
        }
    }

    let mut reverse_distances = vec![usize::MAX; num_v + 1];
    let mut travel_pq: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    reverse_distances[dest] = 0;
    travel_pq.push(Reverse((0, dest)));
    while !travel_pq.is_empty() {
        let Reverse((curr_w, curr_v)) = travel_pq.pop().unwrap();

        if curr_w > reverse_distances[curr_v] {
            continue;
        }
        for (d_w, next_v) in &reverse_edges[curr_v] {
            let next_w = d_w + curr_w;
            if reverse_distances[*next_v] > next_w {
                reverse_distances[*next_v] = next_w;
                travel_pq.push(Reverse((next_w, *next_v)));
            }
        }
    }
    writeln!(
        output,
        "{:?}",
        reverse_distances
            .iter()
            .zip(distances)
            .skip(1)
            .map(|(reverse, distance)| reverse + distance)
            .max()
            .unwrap()
    )
    .unwrap();
    print!("{}", output);
    Ok(())
}
