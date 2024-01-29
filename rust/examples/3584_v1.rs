// 가장 가까운 공통 조상
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let t = input.next().unwrap();
    for _ in 0..t {
        let num_v: usize = input.next().unwrap() as usize;
        let mut edges = vec![vec![]; num_v + 1];
        for _ in 0..num_v - 1 {
            let start_v = input.next().unwrap();
            let end_v = input.next().unwrap();
            edges[end_v].push(start_v);
        }

        let mut visited = vec![false; num_v + 1];

        let leaf_1 = input.next().unwrap();
        let leaf_2 = input.next().unwrap();

        let mut travel_stack = VecDeque::new();
        travel_stack.push_back(leaf_1);
        while !travel_stack.is_empty() {
            let curr_v = travel_stack.pop_back().unwrap();
            visited[curr_v] = true;
            for next_v in &edges[curr_v] {
                travel_stack.push_back(*next_v);
            }
        }

        travel_stack.clear();
        travel_stack.push_back(leaf_2);
        while !travel_stack.is_empty() {
            let curr_v = travel_stack.pop_back().unwrap();
            if visited[curr_v] {
                writeln!(output, "{}", curr_v).unwrap();
                break;
            }
            for next_v in &edges[curr_v] {
                travel_stack.push_back(*next_v);
            }
        }
    }
    print!("{}", output);
    Ok(())
}
