// 절연 구간 최소화

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Weight = usize;
type Edge = (usize, Weight);

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let v1 = input.next().unwrap();
        let v2 = input.next().unwrap();
        let weight: Weight = input.next().unwrap();
        graph[v1].push((v2, weight));
        graph[v2].push((v1, weight));
    }

    let start = input.next().unwrap();
    let end = input.next().unwrap();

    let mut distances = vec![usize::MAX; n + 1];
    distances[start] = 0;

    let mut travel_deq: VecDeque<(usize, Weight)> = VecDeque::new();
    travel_deq.push_front((start, 0));
    travel_deq.push_front((start, 1)); // 시작은 두 타입이 가능하다!

    while let Some((vertex, curr_type)) = travel_deq.pop_front() {
        for &(next_v, edge_type) in &graph[vertex] {
            let next_changes = distances[vertex] + (curr_type ^ edge_type);
            // 가중치가 현재와 같은 지 다른지 여부만 알면 되니까, xor, 따라서 0-1 BFS
            if next_changes < distances[next_v] {
                distances[next_v] = next_changes;
                if curr_type ^ edge_type == 0 {
                    travel_deq.push_front((next_v, edge_type));
                } else {
                    travel_deq.push_back((next_v, edge_type));
                }
            }
        }
    }

    writeln!(output, "{}", distances[end]).unwrap();
    print!("{}", output);
    Ok(())
}
