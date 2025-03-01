// 게임 개발

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;

    let mut build_times = vec![0; n + 1];
    let mut edges: Vec<Vec<usize>> = vec![vec![]; n + 1];

    let mut starts = Vec::new();

    for idx in 1..=n {
        build_times[idx] = input.next().unwrap() as usize;
        for (jdx, val) in input.by_ref().enumerate() {
            if val == -1 {
                if jdx == 0 {
                    starts.push(idx);
                }
                break;
            } else {
                let val = val as usize;
                edges[val].push(idx);
            }
        }
    }

    let mut tot_min_times = vec![0usize; n + 1];

    let mut travel_q = VecDeque::new();
    for start_idx in starts {
        travel_q.push_back((0, start_idx));
    }
    while !travel_q.is_empty() {
        let (curr_t, curr_v) = travel_q.pop_front().unwrap();
        tot_min_times[curr_v] = (curr_t + build_times[curr_v]).max(tot_min_times[curr_v]);

        for &next_v in &edges[curr_v] {
            travel_q.push_back((tot_min_times[curr_v], next_v));
        }
    }
    for tot_min_time in tot_min_times.iter().skip(1) {
        writeln!(output, "{}", tot_min_time)?;
    }
    print!("{}", output);
    Ok(())
}
