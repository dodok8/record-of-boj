// 알고스팟

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_connected_points(
    point: (usize, usize),
    max_h: usize,
    max_w: usize,
    graph: &Vec<Vec<usize>>,
) -> Vec<(usize, (usize, usize))> {
    let mut result = Vec::new();
    for (dh, dw) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let h = point.0 as i32 + dh;
        let w = point.1 as i32 + dw;
        if (0 <= h && h < max_h as i32) && (0 <= w && w < max_w as i32) {
            let h = h as usize;
            let w = w as usize;
            let weight = graph[h][w];
            result.push((weight, (h, w)));
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    let m = input.next().unwrap().parse::<usize>()?;
    let mut graph = Vec::new();
    for _ in 0..m {
        graph.push(
            input
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>(),
        )
    }

    let mut visited = vec![vec![usize::MAX; n]; m];
    visited[0][0] = 0;
    let mut travel_deq = VecDeque::new();
    travel_deq.push_front((0, (0, 0)));
    while !travel_deq.is_empty() {
        let (curr_w, curr_p) = travel_deq.pop_front().unwrap();
        for (weight, point) in get_connected_points(curr_p, m, n, &graph) {
            let next_w = weight + curr_w;
            if next_w < visited[point.0][point.1] {
                visited[point.0][point.1] = next_w;
                if weight == 0 {
                    travel_deq.push_front((next_w, point));
                } else {
                    travel_deq.push_back((next_w, point));
                }
            }
        }
    }
    writeln!(output, "{}", visited[m - 1][n - 1]).unwrap();
    print!("{}", output);
    Ok(())
}
