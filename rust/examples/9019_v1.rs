// DSLR

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_d(num: usize) -> usize {
    num * 2 % 10000
}

fn get_s(num: usize) -> usize {
    (num + 10000 - 1) % 10000
}

fn get_l(num: usize) -> usize {
    num / 1000 + (num % 1000) * 10
}

fn get_r(num: usize) -> usize {
    (num % 10) * 1000 + (num / 10)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let num_test = input.next().unwrap();

    for _ in 0..num_test {
        let mut visited: Vec<Option<(usize, char)>> = vec![None; 10000];
        let start = input.next().unwrap();
        let end = input.next().unwrap();

        let mut travel_q = VecDeque::new();
        travel_q.push_back(start);

        while !travel_q.is_empty() {
            let curr = travel_q.pop_front().unwrap();

            if curr == start {
                visited[start] = Some((start, 'E'));
            }

            if curr == end {
                break;
            }

            for calc in ['D', 'S', 'L', 'R'] {
                let next = match calc {
                    'D' => get_d(curr),
                    'S' => get_s(curr),
                    'L' => get_l(curr),
                    'R' => get_r(curr),
                    _ => panic!(),
                };

                if let Some(_val) = visited[next] {
                    continue;
                } else {
                    visited[next] = Some((curr, calc));
                    travel_q.push_back(next);
                }
            }
        }

        let mut path = Vec::new();
        let mut current = end;

        while current != start {
            let (prev, calc) = visited[current].unwrap();
            path.push(calc);
            current = prev;
        }

        path.reverse();

        for calc in path {
            write!(output, "{}", calc).unwrap();
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
