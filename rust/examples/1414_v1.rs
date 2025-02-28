// 불우이웃 돕기

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Weight = i64;
type Vertex = usize;

fn convert_to_num(letter: char) -> i64 {
    match letter {
        'a'..='z' => (letter as u8 - b'a') as i64 + 1,
        'A'..='Z' => (letter as u8 - b'A') as i64 + 27,
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;

    let mut edges: Vec<Vec<Weight>> = vec![vec![i64::MAX; n]; n];
    let mut sum = 0;

    for start in 0..n {
        for (end, letter) in input.next().unwrap().chars().enumerate() {
            let weight: Weight = convert_to_num(letter);
            sum += weight;
            if weight != 0 {
                edges[start][end] = weight;
            }
        }
    }

    let mut ans = -1;
    for start_idx in 0..n {
        let mut contained = vec![false; n];
        let mut contained_cnt = 0usize;

        let mut pq: BinaryHeap<Reverse<(Weight, Vertex)>> = BinaryHeap::new();
        pq.push(Reverse((0, start_idx)));
        let mut dist = 0;

        while !pq.is_empty() {
            let Reverse((weight, next_v)) = pq.pop().unwrap();

            if contained[next_v] {
                continue;
            }
            contained[next_v] = true;
            contained_cnt += 1;
            dist += weight;

            for idx in 0..n {
                if edges[idx][next_v] == i64::MAX && edges[next_v][idx] == i64::MAX {
                    continue;
                }
                pq.push(Reverse((edges[idx][next_v].min(edges[next_v][idx]), idx)))
            }
        }

        if contained_cnt == n {
            ans = ans.max(sum - dist);
        } else {
            ans = ans.max(-1);
        }
    }

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
