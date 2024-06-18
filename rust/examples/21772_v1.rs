// 가희의 고구마 먹방

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn search(
    curr_p: (usize, usize),
    curr_t: usize,
    curr_s: usize,
    max_t: usize,
    graph: &mut Vec<Vec<char>>,
) -> usize {
    let max_h = graph.len();
    let max_w = graph[0].len();
    if curr_t == max_t {
        curr_s
    } else {
        let mut result = Vec::new();
        let mut positions = HashSet::new();
        positions.insert((curr_p.0.saturating_sub(1), curr_p.1));
        positions.insert((curr_p.0 + 1, curr_p.1));
        positions.insert((curr_p.0, curr_p.1.saturating_sub(1)));
        positions.insert((curr_p.0, curr_p.1 + 1));
        positions.insert(curr_p);
        let unique_positions: Vec<_> = positions.into_iter().collect();
        for (h, w) in unique_positions {
            let mut next_s = curr_s;
            if (h < max_h) && (w < max_w) {
                match graph[h][w] {
                    '#' => continue,
                    'S' => {
                        next_s += 1;
                        graph[h][w] = '.';
                        result.push(search((h, w), curr_t + 1, next_s, max_t, graph));
                        graph[h][w] = 'S';
                    }
                    _ => result.push(search((h, w), curr_t + 1, next_s, max_t, graph)),
                }
            }
        }
        *result.iter().max().unwrap()
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let max_h = input.next().unwrap().parse::<usize>()?;
    let _max_w = input.next().unwrap().parse::<usize>()?;
    let target_t = input.next().unwrap().parse::<usize>()?;

    let mut graph = Vec::new();
    let mut start = (0, 0);
    for idx in 0..max_h {
        let mut row = Vec::new();
        for (jdx, char) in input.next().unwrap().chars().enumerate() {
            if char == 'G' {
                start = (idx, jdx);
            }
            row.push(char);
        }
        graph.push(row);
    }

    writeln!(output, "{}", search(start, 0, 0, target_t, &mut graph)).unwrap();
    print!("{}", output);
    Ok(())
}
