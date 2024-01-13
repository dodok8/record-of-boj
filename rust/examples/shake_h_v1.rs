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
        travel_stack.push_back(start_v);
        visited[start_v] = true;

        let mut continued = true;
        while !travel_stack.is_empty() {
            let curr_v = travel_stack.pop_back().unwrap();
            status_vertex[curr_v].curr += w;
            if status_vertex[curr_v].curr > status_vertex[curr_v].max {
                println!("{}", count - 1);
                continued = false;
                break;
            }
            if curr_v == end_v {
                break;
            }
            for vertex in &graph[curr_v] {
                if !visited[*vertex] {
                    visited[*vertex] = true;
                    travel_stack.push_back(*vertex);
                }
            }
        }

        if !continued {
            break;
        }
    }
    Ok(())
}
