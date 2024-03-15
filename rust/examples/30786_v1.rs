// 홀수 찾아 삼만리

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_combinations(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    fn combinations_helper(
        vec: &[usize],
        n: usize,
        prefix: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if n == 0 {
            result.push(prefix.clone());
            return;
        }

        for (i, &item) in vec.iter().enumerate() {
            let remaining = vec[i + 1..].to_vec();
            prefix.push(item);
            combinations_helper(&remaining, n - 1, prefix, result);
            prefix.pop();
        }
    }

    let mut result = Vec::new();
    let mut prefix = Vec::new();
    combinations_helper(&vec, n, &mut prefix, &mut result);
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut cities = Vec::new();
    for _ in 0..n {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        cities.push((a, b));
    }
    let mut result = VecDeque::new();
    for c in get_combinations((0..(n)).collect::<Vec<usize>>(), 2) {
        let curr = cities[c[0]];
        let next = cities[c[1]];
        let diff = curr.0.abs_diff(next.0) + curr.1.abs_diff(next.1);
        result.push_back(0);
        if diff % 2 == 1 {
            for i in 1..n {
                if (*result.back().unwrap() == c[1] && i == c[0])
                    || (*result.back().unwrap() == c[0] && i == c[1])
                {
                    continue;
                } else {
                    result.push_back(i);
                }
            }
            result.push_back(c[1]);
            break;
        }
    }
    if result.is_empty() {
        writeln!(output, "NO").unwrap();
    } else {
        writeln!(output, "YES").unwrap();
        for item in result.iter() {
            write!(output, "{} ", *item + 1).unwrap();
        }
        writeln!(output, "").unwrap();
    }
    print!("{}", output);
    Ok(())
}
