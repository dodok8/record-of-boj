// 케빈 베이컨의 6단계 법칙
use std::cmp;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::{error::Error, fmt};

type Weight = i32;
type Vertex = i32;
type Edge = (Weight, Vertex);

#[derive(Debug)]
struct MinusCycleError;

impl Error for MinusCycleError {}

impl fmt::Display for MinusCycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Minus Cycle Exists")
    }
}

fn get_floyd_warshall(edges: &Vec<Vec<Edge>>) -> Result<Vec<Vec<i32>>, MinusCycleError> {
    let num_v = edges.len() - 1;
    // 0번째는 구현 편의를 위해 넣은 것이므로 제외한다.
    let mut weights: Vec<Vec<i32>> = vec![vec![i32::MAX; num_v + 1]; num_v + 1];

    for (idx, weight) in weights.iter_mut().enumerate() {
        weight[idx] = 0;
    }

    for start_v in 1..num_v + 1 {
        for (weight, adj_v) in &edges[start_v] {
            if *weight < weights[start_v][*adj_v as usize] {
                weights[start_v][*adj_v as usize] = *weight
            }
        }
    }

    // start_v와 end_v가 같을 때, 그 값이 0 이하면 음수 사이클
    for mid_v in 1..num_v + 1 {
        for end_v in 1..num_v + 1 {
            for start_v in 1..num_v + 1 {
                if weights[start_v][mid_v] == i32::MAX || weights[mid_v][end_v] == i32::MAX {
                    continue;
                }
                weights[start_v][end_v] = cmp::min(
                    weights[start_v][end_v],
                    weights[start_v][mid_v] + weights[mid_v][end_v],
                );
                if start_v == end_v && weights[start_v][end_v] < 0 {
                    return Err(MinusCycleError);
                }
            }
        }
    }
    Ok(weights)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let num_v = input.next().unwrap();
    let num_e = input.next().unwrap();
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; (num_v + 1) as usize];

    for _ in 0..num_e {
        let start: Vertex = input.next().unwrap();
        let end: Vertex = input.next().unwrap();
        edges[start as usize].push((1, end));
        edges[end as usize].push((1, start))
    }

    let mut sum: i32 = i32::MAX;
    let smallest_weights = get_floyd_warshall(&edges)?;
    let mut ans = 0;

    for idx in 1..num_v + 1 {
        if sum > smallest_weights[idx as usize][1..].iter().sum() {
            ans = idx;
            sum = smallest_weights[idx as usize][1..].iter().sum();
        }
    }
    writeln!(output, "{}", ans).unwrap();
    print!("{}", output);
    Ok(())
}
