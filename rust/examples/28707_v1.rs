// 배열 정렬

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Weight = usize;
type Vertex = usize;
type Operation = (usize, usize, Weight);
type Edge = (Weight, Vertex);

fn operate(source: usize, op: Operation, n: usize) -> (usize, Vertex) {
    let a = op.0;
    let b = op.1;
    let temp_1 = source / 10usize.pow(n as u32 - 1 - a as u32) % 10;
    let temp_2 = source / 10usize.pow(n as u32 - 1 - b as u32) % 10;

    (
        op.2,
        source
            - temp_1 * 10usize.pow(n as u32 - 1 - a as u32)
            - temp_2 * 10usize.pow(n as u32 - 1 - b as u32)
            + temp_1 * 10usize.pow(n as u32 - 1 - b as u32)
            + temp_2 * 10usize.pow(n as u32 - 1 - a as u32),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let initial = input
        .by_ref()
        .take(n)
        .map(|x| x - 1)
        .collect::<Vec<usize>>();

    let mut target = initial.clone();
    target.sort_unstable();
    let target: usize = target
        .iter()
        .enumerate()
        .map(|(i, &digit)| digit * 10usize.pow(n as u32 - 1 - i as u32))
        .sum();

    let initial: usize = initial
        .iter()
        .enumerate()
        .map(|(i, &digit)| digit * 10usize.pow(n as u32 - 1 - i as u32))
        .sum();

    let max = 10usize.pow(n as u32);
    let num_o = input.by_ref().next().unwrap();

    let mut operations: Vec<Operation> = Vec::new();

    for _ in 0..num_o {
        operations.push((
            input.next().unwrap() - 1,
            input.next().unwrap() - 1,
            input.next().unwrap(),
        ));
    }

    let mut weights = vec![usize::MAX; max];
    weights[initial] = 0;
    let mut travel_pq: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    travel_pq.push(Reverse((0, initial)));

    while !travel_pq.is_empty() {
        let Reverse((curr_w, curr_v)) = travel_pq.pop().unwrap();

        if curr_w > weights[curr_v] {
            continue;
        }
        for (d_w, next_v) in operations.iter().map(|op| operate(curr_v, *op, n)) {
            let next_w = curr_w + d_w;

            if weights[next_v] > next_w {
                weights[next_v] = next_w;
                travel_pq.push(Reverse((next_w, next_v)));
            }
        }
    }
    if weights[target] == usize::MAX {
        writeln!(output, "-1").unwrap();
    } else {
        writeln!(output, "{}", weights[target]).unwrap();
    }
    print!("{}", output);
    Ok(())
}
