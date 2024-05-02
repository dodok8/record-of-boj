// 문제집
// 위상정렬 문제

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let num_v = input.next().unwrap();
    let num_e = input.next().unwrap();
    let mut in_deg = vec![0; num_v + 1];
    let mut edges: Vec<Vec<usize>> = vec![vec![]; num_v + 1];
    for _ in 0..num_e {
        let start = input.next().unwrap();
        let end = input.next().unwrap();
        in_deg[end] += 1;
        edges[start].push(end);
    }

    let mut pq: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for (idx, &value) in in_deg.iter().enumerate().skip(1) {
        if value == 0 {
            pq.push(Reverse(idx));
        }
    }

    while !pq.is_empty() {
        let Reverse(curr) = pq.pop().unwrap();
        for end in &edges[curr] {
            in_deg[*end] -= 1;
            if in_deg[*end] == 0 {
                pq.push(Reverse(*end));
            }
        }
        edges[curr].clear();
        write!(output, "{} ", curr).unwrap();
    }
    println!("{}", output);
    Ok(())
}
