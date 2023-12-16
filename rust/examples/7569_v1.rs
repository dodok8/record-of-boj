use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_connected_points(
    point: (usize, usize, usize),
    h: i32,
    m: i32,
    n: i32,
    result: &Vec<Vec<Vec<i32>>>,
) -> Vec<(usize, usize, usize)> {
    let mut connected_points: Vec<(usize, usize, usize)> = Vec::new();
    let deltas = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, 0, 1),
        (0, 1, 0),
        (0, -1, 0),
    ];
    for (dh, dm, dn) in deltas {
        let hdx = point.0 as i32 + dh;
        let mdx = point.1 as i32 + dm;
        let ndx = point.2 as i32 + dn;
        if (0 <= hdx && hdx < h)
            && (0 <= mdx && mdx < m)
            && (0 <= ndx && ndx < n)
            && result[hdx as usize][mdx as usize][ndx as usize] == 0
        {
            connected_points.push((hdx as usize, mdx as usize, ndx as usize));
        }
    }
    connected_points
}

fn check_ready(result: &Vec<Vec<Vec<i32>>>) -> bool {
    for hdx in result {
        for mdx in hdx {
            for ndx in mdx {
                if *ndx == 0 {
                    return false;
                }
            }
        }
    }
    true
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let n = input.next().unwrap() as usize;
    let m = input.next().unwrap() as usize;
    let h = input.next().unwrap() as usize;
    let mut board = vec![vec![vec![0; n]; m]; h];
    let mut result = vec![vec![vec![0; n]; m]; h];
    let mut start_points: Vec<(usize, usize, usize)> = Vec::new();
    for hdx in 0..h {
        for jdx in 0..m {
            for idx in 0..n {
                let curr_cell = input.next().unwrap();
                board[hdx][jdx][idx] = curr_cell;
                result[hdx][jdx][idx] = curr_cell;
                if curr_cell == 1 {
                    start_points.push((hdx, jdx, idx));
                }
            }
        }
    }

    let mut travel_queue = VecDeque::from(start_points);

    loop {
        let curr_point = travel_queue.pop_front().unwrap();
        for point in get_connected_points(curr_point, h as i32, m as i32, n as i32, &result) {
            result[point.0][point.1][point.2] =
                result[curr_point.0][curr_point.1][curr_point.2] + 1;
            travel_queue.push_back(point);
        }

        if travel_queue.is_empty() {
            if check_ready(&result) {
                writeln!(
                    output,
                    "{}",
                    result[curr_point.0][curr_point.1][curr_point.2] - 1
                )
                .unwrap();
            } else {
                writeln!(output, "-1").unwrap();
            }
            break;
        }
    }

    print!("{}", output);
    Ok(())
}
