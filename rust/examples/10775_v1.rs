// 공항

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
    let num_g = input.next().unwrap();
    let mut parents = (0..=num_g).collect::<Vec<usize>>();

    let num_p = input.next().unwrap();
    let planes = input.take(num_p).collect::<Vec<usize>>();

    for (idx, &plane) in planes.iter().enumerate() {
        if find_parent(&mut parents, plane) == 0 {
            writeln!(output, "{}", idx).unwrap();
            break;
        } else {
            union(
                find_parent(&mut parents, plane) - 1,
                find_parent(&mut parents, plane),
                &mut parents,
            )
        }
    }

    print!("{}", output);
    Ok(())
}
