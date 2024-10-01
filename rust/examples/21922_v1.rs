// 학부 연구생 민상

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Direction = usize;
type Point = (usize, usize);

fn get_connected_points(
    point: Point,
    direction: Direction,
    graph: &Vec<Vec<usize>>,
    visited: &Vec<Vec<Vec<bool>>>, // 방문 체크는 큐에 넣을 때 하자. 에어컨 체크도 해야하니까
) -> Option<(Point, Direction)> {
    let n = graph.len();
    let m = graph[0].len();
    let next_val: Option<(Point, Direction)> = match graph[point.0][point.1] {
        0 | 9 => match direction {
            0 => {
                if point.0 < n - 1 {
                    Some(((point.0 + 1, point.1), direction))
                } else {
                    None
                }
            }
            1 => {
                if point.1 > 0 {
                    Some(((point.0, point.1 - 1), direction))
                } else {
                    None
                }
            }
            2 => {
                if point.0 > 0 {
                    Some(((point.0 - 1, point.1), direction))
                } else {
                    None
                }
            }
            3 => {
                if point.1 < m - 1 {
                    Some(((point.0, point.1 + 1), direction))
                } else {
                    None
                }
            }
            _ => unreachable!(),
        },
        1 => {
            // | 칸
            match direction {
                0 => {
                    if point.0 < n - 1 {
                        Some(((point.0 + 1, point.1), 0))
                    } else {
                        None
                    }
                }
                1 => {
                    if point.1 < m - 1 {
                        Some(((point.0, point.1 + 1), 3))
                    } else {
                        None
                    }
                }
                2 => {
                    if point.0 > 0 {
                        Some(((point.0 - 1, point.1), 2))
                    } else {
                        None
                    }
                }
                3 => {
                    if point.1 > 0 {
                        Some(((point.0, point.1 - 1), 1))
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            }
        }
        2 => {
            // - 칸
            match direction {
                0 => {
                    if point.0 > 0 {
                        Some(((point.0 - 1, point.1), 2))
                    } else {
                        None
                    }
                }
                1 => {
                    if point.1 > 0 {
                        Some(((point.0, point.1 - 1), direction))
                    } else {
                        None
                    }
                }
                2 => {
                    if point.0 < n - 1 {
                        Some(((point.0 + 1, point.1), 0))
                    } else {
                        None
                    }
                }
                3 => {
                    if point.1 < m - 1 {
                        Some(((point.0, point.1 + 1), direction))
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            }
        }
        3 => {
            // / 칸
            match direction {
                0 => {
                    if point.1 > 0 {
                        Some(((point.0, point.1 - 1), 1))
                    } else {
                        None
                    }
                }
                1 => {
                    if point.0 < n - 1 {
                        Some(((point.0 + 1, point.1), 0))
                    } else {
                        None
                    }
                }
                2 => {
                    if point.1 < m - 1 {
                        Some(((point.0, point.1 + 1), 3))
                    } else {
                        None
                    }
                }
                3 => {
                    if point.0 > 0 {
                        Some(((point.0 - 1, point.1), 2))
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            }
        }
        4 => {
            // \ 칸
            match direction {
                0 => {
                    if point.1 < m - 1 {
                        Some(((point.0, point.1 + 1), 3))
                    } else {
                        None
                    }
                }
                1 => {
                    if point.0 > 0 {
                        Some(((point.0 - 1, point.1), 2))
                    } else {
                        None
                    }
                }
                2 => {
                    if point.1 > 0 {
                        Some(((point.0, point.1 - 1), 1))
                    } else {
                        None
                    }
                }
                3 => {
                    if point.0 < n - 1 {
                        Some(((point.0 + 1, point.1), 0))
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };

    if let Some((p, d)) = next_val {
        if !visited[d][p.0][p.1] {
            Some((p, d))
        } else {
            None
        }
    } else {
        None
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let mut starts = vec![];
    let mut graph = vec![vec![0; m]; n];
    let mut visited = vec![vec![vec![false; m]; n]; 4];
    let mut aircon = vec![vec![false; m]; n];
    for hdx in 0..n {
        for wdx in 0..m {
            let cell = input.next().unwrap();
            if cell == 9 {
                starts.push((hdx, wdx));
            }
            graph[hdx][wdx] = cell;
        }
    }

    let mut travel_q = VecDeque::new();
    for start in starts {
        for dir in 0..4 {
            visited[dir][start.0][start.1] = true;
        }
        aircon[start.0][start.1] = true;
        if start.0 > 0 {
            travel_q.push_back(((start.0 - 1, start.1), 2));
        }
        if start.0 < n - 1 {
            travel_q.push_back(((start.0 + 1, start.1), 0));
        }
        if start.1 > 0 {
            travel_q.push_back(((start.0, start.1 - 1), 1));
        }
        if start.1 < m - 1 {
            travel_q.push_back(((start.0, start.1 + 1), 3));
        }
    }

    while !travel_q.is_empty() {
        let (point, direction) = travel_q.pop_front().unwrap();
        aircon[point.0][point.1] = true;
        if let Some((next_point, next_dir)) =
            get_connected_points(point, direction, &graph, &visited)
        {
            visited[next_dir][next_point.0][next_point.1] = true;
            travel_q.push_back((next_point, next_dir));
        }
    }

    writeln!(
        output,
        "{}",
        aircon
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&value| value)
            .count()
    )?;
    print!("{}", output);
    Ok(())
}
