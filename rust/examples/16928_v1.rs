use std::cmp::min;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_ladder = input.next().unwrap();
    let num_snakes = input.next().unwrap();

    let mut edges = [None; 101];
    for _ in 0..(num_ladder + num_snakes) {
        let curr_cell = input.next().unwrap();
        edges[curr_cell] = Some(input.next().unwrap());
    }

    let get_connected = |num: usize| {
        (num + 1..min(num + 7, 101))
            .map(|next_v| edges[next_v].unwrap_or(next_v))
            .collect::<Vec<usize>>()
    };

    let mut travel_q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited = [false; 101];
    travel_q.push_back((1, 0));
    while !travel_q.is_empty() {
        let (curr_v, depth) = travel_q.pop_front().unwrap();
        visited[curr_v] = true;
        if curr_v == 100 {
            writeln!(output, "{}", depth).unwrap();
            break;
        }
        for next_v in get_connected(curr_v) {
            if !visited[next_v] {
                travel_q.push_back((next_v, depth + 1));
            }
        }
    }
    print!("{}", output);

    Ok(())
}
