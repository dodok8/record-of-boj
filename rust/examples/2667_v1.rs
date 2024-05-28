// 단지 번호 붙이기

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_connected_points(
    point: (usize, usize),
    apartments: &mut Vec<Vec<usize>>,
    visited: &Vec<Vec<bool>>,
) -> Vec<(usize, usize)> {
    let mut connected_points: Vec<(usize, usize)> = Vec::new();
    let curr_cell = apartments[point.0][point.1];
    for (h, w) in [
        (point.0.saturating_sub(1), point.1),
        (point.0 + 1, point.1),
        (point.0, point.1.saturating_sub(1)),
        (point.0, point.1 + 1),
    ] {
        let n = apartments.len();
        if (h < n) && (w < n) && !visited[h][w] && curr_cell == apartments[h][w] {
            connected_points.push((h, w));
        }
    }

    connected_points
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    let mut apartments = vec![vec![0; n]; n];
    let mut visited = vec![vec![false; n]; n];
    for (idx, row) in input.enumerate() {
        for (jdx, cell) in row.chars().enumerate() {
            let cell = cell.to_digit(10).unwrap() as usize;

            apartments[idx][jdx] = cell;
            if cell == 0 {
                visited[idx][jdx] = true;
            }
        }
    }

    let mut travel_q = VecDeque::new();
    let mut num_apart = Vec::new();
    for idx in 0..n {
        for jdx in 0..n {
            if !visited[idx][jdx] {
                visited[idx][jdx] = true;
                let mut num_cell = 0;
                travel_q.push_back((idx, jdx));
                while !travel_q.is_empty() {
                    let point = travel_q.pop_front().unwrap();

                    num_cell += 1;

                    for next_point in get_connected_points(point, &mut apartments, &visited) {
                        visited[next_point.0][next_point.1] = true;
                        // 방문 체크 시점을 외우자!
                        travel_q.push_back(next_point);
                    }
                }
                num_apart.push(num_cell);
            }
        }
    }

    num_apart.sort_unstable();
    writeln!(output, "{}", num_apart.len()).unwrap();
    for num in num_apart {
        writeln!(output, "{}", num).unwrap();
    }
    print!("{}", output);
    Ok(())
}
