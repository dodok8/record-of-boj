// 신촌 도로망 관리와 쿼리

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
    let mut input = input.split_ascii_whitespace();

    let num_v = input.next().unwrap().parse::<usize>()?;
    let num_e = input.next().unwrap().parse::<usize>()?;
    let num_test = input.next().unwrap().parse::<usize>()?;
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];

    for _ in 0..num_e {
        let start = input.next().unwrap().parse::<usize>()?;
        let end = input.next().unwrap().parse::<usize>()?;
        let weight: usize = match input.next().unwrap().chars().next().unwrap() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            _ => unreachable!(),
        };
        edges[start].push((weight, end));
        edges[end].push((weight, start));
    }

    for _ in 0..num_test {
        let cost = [
            input.next().unwrap().parse::<usize>()?,
            input.next().unwrap().parse::<usize>()?,
            input.next().unwrap().parse::<usize>()?,
            input.next().unwrap().parse::<usize>()?,
            input.next().unwrap().parse::<usize>()?,
        ];
        let mut contained = vec![false; num_v + 1];
        let mut edges_pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
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
                    edges_pq.push(Reverse((cost[next_w], next_v)));
                }
            }
        }
        writeln!(output, "{}", dist).unwrap();
    }
    print!("{}", output);
    Ok(())
}
