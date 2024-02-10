// 슬슬 가지를 먹지 않으면 죽는다
// MST를 구성했을 때, 빠진 수 중 가장 작은 걸 출력하면 된다.

use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
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
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];

    for _ in 0..num_e {
        let start = input.next().unwrap();
        let end = input.next().unwrap();
        let weight: Weight = input.next().unwrap();
        edges[start].push((weight, end));
        edges[end].push((weight, start));
    }

    let mut contained = vec![false; num_v + 1];
    let mut dates: BTreeSet<usize> = BTreeSet::new();
    for i in 1..=num_v {
        dates.insert(i);
    }
    let mut result_dates = BTreeSet::new();
    let mut edges_pq: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    edges_pq.push(Reverse((0, 1)));
    while !edges_pq.is_empty() {
        let Reverse((curr_w, curr_v)) = edges_pq.pop().unwrap();
        if contained[curr_v] {
            continue;
        }
        contained[curr_v] = true;
        result_dates.insert(curr_w);
        for &(next_w, next_v) in &edges[curr_v] {
            if !contained[next_v] {
                edges_pq.push(Reverse((next_w, next_v)));
            }
        }
    }

    let ans = dates
        .difference(&result_dates)
        .cloned()
        .collect::<Vec<usize>>();
    writeln!(output, "{}", ans[0]).unwrap();
    print!("{}", output);
    Ok(())
}
