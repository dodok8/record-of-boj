// 얼음깨기 펭귄

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
    let targets = 0..input.next().unwrap();
    let start = input.next().unwrap() - 1;

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); num_v];

    for _ in 0..(num_v - 1) {
        let start = input.next().unwrap() - 1;
        let end = input.next().unwrap() - 1;

        graph[start].push(end);
        graph[end].push(start);
    }

    let mut visited = vec![-1; num_v];

    visited[start] = 0;

    let mut travel_q = VecDeque::new();
    travel_q.push_back((start, visited[start]));
    while !travel_q.is_empty() {
        let (curr_v, curr_w) = travel_q.pop_front().unwrap();

        for &next_v in &graph[curr_v] {
            if visited[next_v] == -1 {
                visited[next_v] = curr_w + 1;
                travel_q.push_back((next_v, visited[next_v]));
            }
        }
    }

    let mut distances = targets.map(|idx| visited[idx]).collect::<Vec<i32>>();
    distances.sort_unstable();
    writeln!(output, "{}", num_v as i32 - distances[0] - distances[1] - 1)?;
    print!("{}", output);
    Ok(())
}
