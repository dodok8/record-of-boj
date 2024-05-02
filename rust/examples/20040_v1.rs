// 사이클 게임

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
    let m = input.next().unwrap();
    let mut parents = (0..n + 1).collect::<Vec<usize>>();
    let mut result = 0;
    for idx in 1..=m {
        let a = input.next().unwrap() + 1;
        let b = input.next().unwrap() + 1;

        if find_parent(&mut parents, a) == find_parent(&mut parents, b) {
            result = idx;
            break;
        }
        union(a, b, &mut parents);
    }
    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
