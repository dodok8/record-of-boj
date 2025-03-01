// 게임 개발

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;

    let mut build_times = vec![0; n + 1];
    let mut edges: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut in_degrees = vec![0; n + 1];

    for idx in 1..=n {
        build_times[idx] = input.next().unwrap() as usize;
        for val in input.by_ref() {
            if val == -1 {
                break;
            } else {
                let val = val as usize;
                edges[val].push(idx);
                in_degrees[idx] += 1;
            }
        }
    }

    let mut tot_min_times = vec![0usize; n + 1];
    let mut travel_q = VecDeque::new();

    for idx in 1..=n {
        if in_degrees[idx] == 0 {
            travel_q.push_back(idx);
            tot_min_times[idx] = build_times[idx];
        }
    }

    while !travel_q.is_empty() {
        let curr_v = travel_q.pop_front().unwrap();
        for &next_v in &edges[curr_v] {
            tot_min_times[next_v] =
                tot_min_times[next_v].max(tot_min_times[curr_v] + build_times[next_v]);

            in_degrees[next_v] -= 1;
            if in_degrees[next_v] == 0 {
                travel_q.push_back(next_v);
            }
        }
    }
    for tot_min_time in tot_min_times.iter().skip(1) {
        writeln!(output, "{}", tot_min_time)?;
    }
    print!("{}", output);
    Ok(())
}
