// 전력난

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

    loop {
        let num_v = input.next().unwrap();
        let num_e = input.next().unwrap();
        if num_v == 0 && num_e == 0 {
            break;
        }
        let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];
        let mut total = 0;

        for _ in 0..num_e {
            let start = input.next().unwrap();
            let end = input.next().unwrap();
            let weight: Weight = input.next().unwrap();
            edges[start].push((weight, end));
            edges[end].push((weight, start));
            total += weight;
        }

        let mut contained = vec![false; num_v + 1];
        let mut edges_pq: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
        let mut dist = 0;
        edges_pq.push(Reverse((0, 1)));
        while !edges_pq.is_empty() {
            let Reverse((curr_w, curr_v)) = edges_pq.pop().unwrap();
            if contained[curr_v] {
                continue;
            }
            contained[curr_v] = true;
            dist += curr_w;
            for &(next_w, next_v) in &edges[curr_v] {
                if !contained[next_v] {
                    edges_pq.push(Reverse((next_w, next_v)));
                }
            }
        }
        writeln!(output, "{}", total - dist).unwrap();
    }
    print!("{}", output);
    Ok(())
}
