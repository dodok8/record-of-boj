// 일루미네이션
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_adj_count_covered(
    curr_v: &(usize, usize),
    building: &Vec<Vec<bool>>,
) -> (Vec<(usize, usize)>, usize) {
    let mut result = Vec::new();
    let mut count = 0;
    let y = curr_v.0;
    let x = curr_v.1;
    let height = building.len();
    let width = building[0].len();
    let deltas = if y % 2 == 1 {
        [(-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0), (1, 1)]
    } else {
        [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 0)]
    };
    for (dy, dx) in deltas {
        let new_y = (y as i32) + dy;
        let new_x = (x as i32) + dx;
        if new_y >= 0 && new_x >= 0 && new_y < (height as i32) && new_x < (width as i32) {
            if building[new_y as usize][new_x as usize] {
                count += 1;
            } else {
                result.push((new_y as usize, new_x as usize));
            }
        }
    }
    (result, count)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let width = input.next().unwrap();
    let height = input.next().unwrap();
    let mut building = vec![vec![false; width + 2]; height + 2];
    let mut visited = vec![vec![false; width + 2]; height + 2];

    for idx in 1..(height + 1) {
        for jdx in 1..(width + 1) {
            if let Some(1) = input.next() {
                building[idx][jdx] = true;
            }
        }
    }
    let mut count_covered: usize = 0;
    let mut travel_q: VecDeque<(usize, usize)> = VecDeque::new();
    travel_q.push_back((0_usize, 0_usize));
    while !travel_q.is_empty() {
        let curr_v = travel_q.pop_front().unwrap();
        if visited[curr_v.0][curr_v.1] {
            continue;
        }
        visited[curr_v.0][curr_v.1] = true;
        let (adj_vertexes, covered) = get_adj_count_covered(&curr_v, &building);
        count_covered += covered;
        for adj_v in adj_vertexes {
            if !visited[adj_v.0][adj_v.1] {
                travel_q.push_back(adj_v);
            }
        }
    }
    writeln!(output, "{}", count_covered).unwrap();
    print!("{}", output);
    Ok(())
}
