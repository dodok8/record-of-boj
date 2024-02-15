// 여러분의 다리가 되어 드리겠습니다!

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn find_parent(parents: &mut Vec<usize>, idx: usize) -> usize {
    if idx != parents[idx] {
        parents[idx] = find_parent(parents, parents[idx]);
    }
    parents[idx]
}

fn union(x: usize, y: usize, parents: &mut Vec<usize>) {
    let x = find_parent(parents, x);
    let y = find_parent(parents, y);
    if x < y {
        parents[y] = x;
    } else {
        parents[x] = y;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut parents: Vec<usize> = (0..(n + 1)).collect();

    for _ in 1..=(n - 2) {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        union(a, b, &mut parents);
    }

    for (i, parent) in parents.iter().enumerate().skip(1) {
        if i == *parent {
            write!(output, "{} ", i).unwrap();
        }
    }

    println!("{}", output);
    Ok(())
}
