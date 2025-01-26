// Your life

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
    let num_e = input.next().unwrap();

    let mut graph = vec![vec![]; num_v + 1];

    for _ in 0..num_e {
        let x = input.next().unwrap();
        let y = input.next().unwrap();
        graph[x].push(y);
    }

    let mut travel_q = VecDeque::new();
    travel_q.push_back((0_usize, 1_usize));
    let mut visited = vec![usize::MAX; num_v + 1];
    visited[1] = 0;
    while !travel_q.is_empty() {
        let (dist, v) = travel_q.pop_front().unwrap();
        for &next_v in &graph[v] {
            if visited[next_v] == usize::MAX {
                visited[next_v] = dist + 1;
                travel_q.push_back((dist + 1, next_v));
            }
        }
    }

    writeln!(output, "{}", visited[num_v])?;
    print!("{}", output);
    Ok(())
}
