// 대역폭 관리

use std::collections::VecDeque;
use std::error::Error;
use std::io::{stdin, Read};
#[derive(Clone)]
struct Traffic {
    curr: usize,
    max: usize,
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_v = input.next().unwrap();
    let m = input.next().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; num_v + 1];
    for _ in 0..num_v - 1 {
        let v1 = input.next().unwrap();
        let v2 = input.next().unwrap();
        graph[v1].push(v2);
        graph[v2].push(v1);
    }
    let mut status_vertex = vec![Traffic { curr: 0, max: 0 }];
    for _ in 0..num_v {
        status_vertex.push(Traffic {
            curr: 0,
            max: input.next().unwrap(),
        });
    }

    for count in 1..(m + 1) {
        let start_v = input.next().unwrap();
        let end_v = input.next().unwrap();
        let w = input.next().unwrap();

        let mut travel_stack: VecDeque<usize> = VecDeque::new();
        let mut visited = vec![false; num_v + 1];
        let mut parent = vec![0; num_v + 1];

        travel_stack.push_back(start_v);
        visited[start_v] = true;
        let mut continued = true;

        while !travel_stack.is_empty() {
            let curr_v = travel_stack.pop_front().unwrap();
            if curr_v == end_v {
                break;
            }
            for vertex in &graph[curr_v] {
                if !visited[*vertex] {
                    visited[*vertex] = true;
                    parent[*vertex] = curr_v;
                    travel_stack.push_back(*vertex);
                }
            }
        }

        let mut curr_v = end_v;

        // 백트래킹 하며 값 업데이트
        while curr_v != 0 {
            status_vertex[curr_v].curr += w;
            if status_vertex[curr_v].curr > status_vertex[curr_v].max {
                continued = false;
                break;
            }
            curr_v = parent[curr_v];
        }
        // 대역폭을 더 사용할 수 없을 때 출력 후 종료
        if !continued {
            println!("{}", count - 1);
            break;
        }
    }
    Ok(())
}
