// 귀엽기만 한 게 아닌 한별 양

use std::collections::VecDeque;
use std::error::Error;
use std::io::{stdin, Read};

fn get_connected_points(
    parent: (usize, usize),
    point: (usize, usize),
    hanbyeol: usize,
    max_h: i32,
    max_w: i32,
    map: &Vec<Vec<i32>>,
    parents: &Vec<Vec<Vec<bool>>>,
) -> Vec<(usize, usize)> {
    let mut connected_points: Vec<(usize, usize)> = Vec::new();
    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (dh, dw) in deltas {
        let h = point.0 as i32 + dh;
        let w = point.1 as i32 + dw;
        if (0 <= h && h < max_h) && (0 <= w && w < max_w) {
            let h = h as usize;
            let w = w as usize;
            let curr_p = map[h][w];

            match curr_p {
                -1 => continue,
                _ => {
                    if parent == (h, w) {
                        continue;
                    }

                    if parents[h][w][parent.0 * h + parent.1]
                        && parents[h][w][point.0 * h + point.1]
                    {
                        continue;
                    }

                    let mut sum = curr_p;
                    sum += map[point.0][point.1];
                    sum += map[parent.0][parent.1];
                    if hanbyeol >= sum as usize {
                        connected_points.push((h, w))
                    }
                }
            }
        }
    }

    connected_points
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let height = str::parse::<usize>(input.next().unwrap()).unwrap();
    let width = str::parse::<usize>(input.next().unwrap()).unwrap();
    let hanbyeol = str::parse::<usize>(input.next().unwrap()).unwrap();

    let mut map: Vec<Vec<i32>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for idx in 0..height {
        map.push(Vec::new());
        for (jdx, letter) in input.next().unwrap().chars().enumerate() {
            match letter {
                'S' => {
                    start = (idx, jdx);
                    map[idx].push(0);
                }
                'H' => {
                    end = (idx, jdx);
                    map[idx].push(0);
                }
                'X' => {
                    map[idx].push(-1);
                }
                _ => map[idx].push(letter.to_digit(10).unwrap() as i32),
            }
        }
    }

    let mut parents: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; height * (height - 1) + width]; width]; height];
    let mut travel_queue = VecDeque::new();
    parents[start.0][start.1][start.0 * height + start.1] = true;
    travel_queue.push_back((0, start, start));
    let mut length: i32 = -1;
    while !travel_queue.is_empty() {
        let (curr_d, parent, curr_p) = travel_queue.pop_front().unwrap();

        if curr_p == end {
            length = curr_d;
            break;
        }
        for point in get_connected_points(
            parent,
            curr_p,
            hanbyeol,
            height as i32,
            width as i32,
            &map,
            &parents,
        ) {
            parents[point.0][point.1][parent.0 * height + parent.1] = true;
            parents[point.0][point.1][curr_p.0 * height + curr_p.1] = true;
            travel_queue.push_back((curr_d + 1, curr_p, point));
        }
    }

    println!("{}", length);
    Ok(())
}
