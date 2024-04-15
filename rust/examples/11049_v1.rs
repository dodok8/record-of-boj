// 행렬 곱셈 순서

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_mul_count(
    start: usize,
    end: usize,
    cache: &mut Vec<Vec<usize>>,
    matrices: &Vec<(usize, usize)>,
) -> usize {
    if start == end {
        0
    } else if end == start + 1 {
        if cache[start][end] == 0 {
            cache[start][end] = matrices[start].0 * matrices[start].1 * matrices[end].1;
        }
        cache[start][end]
    } else {
        if cache[start][end] == 0 {
            let mut results: Vec<usize> = Vec::new();
            for idx in start..end {
                let mut temp = get_mul_count(start, idx, cache, matrices)
                    + get_mul_count(idx + 1, end, cache, matrices);

                temp += matrices[start].0 * matrices[idx].1 * matrices[end].1;

                results.push(temp);
            }
            cache[start][end] = *results.iter().min().unwrap();
        }

        cache[start][end]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut matrices: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        matrices.push((input.next().unwrap(), input.next().unwrap()));
    }
    let mut cache = vec![vec![0_usize; n]; n];
    writeln!(output, "{}", get_mul_count(0, n - 1, &mut cache, &matrices)).unwrap();
    print!("{}", output);
    Ok(())
}
