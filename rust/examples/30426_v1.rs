// Rebirth

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_dim = input.next().unwrap();
    let start_dim = input.next().unwrap();
    let num_problem = input.next().unwrap();
    let mut problem_info = Vec::new();
    for _ in 0..num_problem {
        problem_info.push((input.next().unwrap(), input.next().unwrap()));
    }
    let mut is_stable = vec![true; num_dim];
    for _ in 0..input.next().unwrap() {
        is_stable[input.next().unwrap()] = false;
    }

    let mut travel_q = VecDeque::new();
    travel_q.push_back((start_dim, 0_usize));
    let mut visited_utopia = false;

    while !travel_q.is_empty() {
        let (curr_dim, curr_problem) = travel_q.pop_front().unwrap();

        if !is_stable[curr_dim] {
            continue;
        }

        if curr_problem == num_problem {
            if curr_dim == 0 {
                visited_utopia = true;
                break;
            } else {
                continue;
            }
        }

        let jump_d = problem_info[curr_problem].0;
        let next_dim = (curr_dim + jump_d) % num_dim;
        let next_problem = curr_problem + 1;


        travel_q.push_back((next_dim, next_problem));

        let jump_d = problem_info[curr_problem].1;
        let next_dim = (curr_dim + jump_d) % num_dim;

        travel_q.push_back((next_dim, next_problem));

    }

    if visited_utopia {
        writeln!(output, "utopia").unwrap();
    } else {
        writeln!(output, "dystopia").unwrap();
    }
    print!("{}", output);
    Ok(())
}
