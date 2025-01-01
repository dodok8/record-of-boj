// 아기 상어

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Eq, PartialEq)]
struct Info(usize, (usize, usize));

impl Ord for Info {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .cmp(&other.0)
            .then_with(|| self.1 .0.cmp(&other.1 .0))
            .then_with(|| self.1 .1.cmp(&other.1 .1))
    }
}

impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_connected_points(
    point: &(usize, usize),
    graph: &mut [Vec<usize>],
    visited: &[Vec<bool>],
) -> Vec<(usize, usize)> {
    let mut connected_points: Vec<(usize, usize)> = Vec::new();
    for (h, w) in [
        (point.0.saturating_sub(1), point.1),
        (point.0, point.1.saturating_sub(1)),
        (point.0, point.1 + 1),
        (point.0 + 1, point.1),
    ] {
        let n = graph.len();
        if (h < n) && (w < n) && !visited[h][w] {
            connected_points.push((h, w));
        }
    }

    connected_points
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    let mut graph = vec![vec![0_usize; n]; n];

    let mut start = (0, 0);
    for idx in 0..n {
        for jdx in 0..n {
            graph[idx][jdx] = input.next().unwrap();

            if graph[idx][jdx] == 9 {
                start = (idx, jdx);
                graph[idx][jdx] = 0;
            }
        }
    }

    let mut time = 0_usize;
    let mut curr_size = 2_usize;
    let mut eaten = 0_usize;
    let ans = loop {
        let mut travel_q: VecDeque<Info> = VecDeque::new();
        travel_q.push_back(Info(0, start));
        let mut visited = vec![vec![false; n]; n];
        visited[start.0][start.1] = true;

        let mut fishes: BinaryHeap<Reverse<Info>> = BinaryHeap::new();
        let mut min_dist = usize::MAX; // 최소 거리 추적

        while !travel_q.is_empty() {
            let Info(d_time, point) = travel_q.pop_front().unwrap();

            // 현재 발견한 물고기보다 더 먼 거리는 탐색할 필요가 없음.
            if d_time > min_dist {
                break;
            }

            for next_point in get_connected_points(&point, &mut graph, &visited) {
                if graph[next_point.0][next_point.1] <= curr_size {
                    visited[next_point.0][next_point.1] = true;

                    if graph[next_point.0][next_point.1] > 0
                        && graph[next_point.0][next_point.1] < curr_size
                    {
                        min_dist = d_time + 1;
                        fishes.push(Reverse(Info(d_time + 1, next_point)));
                    }

                    travel_q.push_back(Info(d_time + 1, next_point));
                }
            }
        }

        if fishes.is_empty() {
            break time;
        } else {
            let Reverse(Info(depth, point)) = fishes.pop().unwrap();

            time += depth;
            start = point;
            eaten += 1;
            if eaten == curr_size {
                curr_size += 1;
                eaten = 0;
            }
            graph[point.0][point.1] = 0;
        }
    };
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
