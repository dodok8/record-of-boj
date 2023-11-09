use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_v = input.next().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; num_v + 1];
    let mut visited = vec![false; num_v + 1];
    let mut parents: Vec<usize> = vec![0; num_v + 1];
    visited[0] = true;
    for _ in 0..num_v - 1 {
        let v1 = input.next().unwrap();
        let v2 = input.next().unwrap();
        graph[v1].push(v2);
        graph[v2].push(v1);
    }
    let mut travel_queue: VecDeque<usize> = VecDeque::new();
    travel_queue.push_back(1);
    while !travel_queue.is_empty() {
        let curr_v = travel_queue.pop_front().unwrap();
        visited[curr_v] = true;
        for v in &graph[curr_v] {
            if !visited[*v] {
                parents[*v] = curr_v;
                travel_queue.push_back(*v);
            }
        }
    }
    for v in parents.iter().skip(2) {
        writeln!(output, "{}", v).unwrap();
    }
    print!("{}", output);
    Ok(())
}
