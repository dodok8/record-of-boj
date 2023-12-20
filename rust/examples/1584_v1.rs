// 게임
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Clone, Copy)]
enum Zone {
    Safe,
    Danger,
    Dead,
}

type Point = (usize, usize);

fn update_zones(
    input: &mut impl Iterator<Item = usize>,
    graph: &mut Vec<Vec<Zone>>,
    num: usize,
    zone: Zone,
) {
    for _ in 0..num {
        let (mut x1, mut y1, mut x2, mut y2) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );

        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }

        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }

        for idx in x1..=x2 {
            for jdx in y1..=y2 {
                graph[idx][jdx] = zone;
            }
        }
    }
}

fn get_connected_points(point: Point, graph: &Vec<Vec<Zone>>) -> Vec<(usize, Point)> {
    let mut connected_points: Vec<(usize, Point)> = Vec::new();
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dx, dy) in deltas {
        let target_x = point.0 as i32 + dx;
        let target_y = point.1 as i32 + dy;
        if (0..501).contains(&target_x) && (0..501).contains(&target_y) {
            let target_x = target_x as usize;
            let target_y = target_y as usize;

            match graph[target_x][target_y] {
                Zone::Danger => connected_points.push((1, (target_x, target_y))),
                Zone::Safe => connected_points.push((0, (target_x, target_y))),
                Zone::Dead => continue,
            }
        }
    }
    connected_points
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut graph = vec![vec![Zone::Safe; 501]; 501];
    let mut result = vec![vec![usize::MAX; 501]; 501];

    let num_danger = input.next().unwrap();
    update_zones(&mut input, &mut graph, num_danger, Zone::Danger);

    let num_dead = input.next().unwrap();
    update_zones(&mut input, &mut graph, num_dead, Zone::Dead);

    let mut travel_pq: BinaryHeap<Reverse<(usize, Point)>> = BinaryHeap::new();
    result[0][0] = 0;
    travel_pq.push(std::cmp::Reverse((0, (0, 0))));

    while !travel_pq.is_empty() {
        let std::cmp::Reverse((curr_weight, curr_point)) = travel_pq.pop().unwrap();

        if curr_weight > result[curr_point.0][curr_point.1] {
            continue;
        }

        for (weight, next_point) in get_connected_points(curr_point, &graph) {
            if result[next_point.0][next_point.1] > curr_weight + weight {
                result[next_point.0][next_point.1] = curr_weight + weight;
                travel_pq.push(std::cmp::Reverse((curr_weight + weight, next_point)));
            }
        }
    }
    if result[500][500] == usize::MAX {
        writeln!(output, "-1").unwrap();
    } else {
        writeln!(output, "{}", result[500][500]).unwrap();
    }

    print!("{}", output);

    Ok(())
}
