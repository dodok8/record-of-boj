// 구호물자
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

    // for (idx, weight) in weights.iter_mut().enumerate() {
    //     weight[idx] = 0;
    // }

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
    let num_v = input.next().unwrap() as usize;
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];

    for start in 1..num_v {
        let num_e = input.next().unwrap();
        for _ in 0..num_e {
            edges[start].push((1, input.next().unwrap()));
        }
    }

    let smallest_weights = get_floyd_warshall(&edges)?;
    // println!("{:?}", smallest_weights);
    let mut cycle = false;

    for idx in 2..num_v {
        // println!(
        //     "{} {} {}",
        //     smallest_weights[1][idx], smallest_weights[idx][num_v], smallest_weights[1][num_v]
        // );
        if smallest_weights[1][num_v] != i32::MAX
            && smallest_weights[1][idx] != i32::MAX
            && smallest_weights[idx][num_v] != i32::MAX
            && smallest_weights[idx][idx] != i32::MAX
        {
            cycle = true;
            break;
        }
    }
    if cycle {
        writeln!(output, "CYCLE").unwrap();
    } else {
        writeln!(output, "NO CYCLE").unwrap();
    }
    print!("{}", output);
    Ok(())
}
