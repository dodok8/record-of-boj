// 죽음의 게임

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let k = input.next().unwrap();
    let mut distances = vec![-1; n];
    let mut travel_q = VecDeque::new();
    travel_q.push_back((0usize, 0));
    let targets = input.take(n).collect::<Vec<usize>>();
    while !travel_q.is_empty() {
        let (curr_idx, curr_distance) = travel_q.pop_front().unwrap();

        if curr_idx == k {
            writeln!(output, "{}", curr_distance)?;
            break;
        } else if distances[curr_idx] == -1 {
            distances[curr_idx] = curr_distance;
            travel_q.push_back((targets[curr_idx], curr_distance + 1));
        } else {
            writeln!(output, "-1")?;
            break;
        }
    }

    print!("{}", output);
    Ok(())
}
