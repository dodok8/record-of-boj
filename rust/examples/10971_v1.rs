// 외판원 순회 2

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_permutations(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    fn permutations_helper(
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
            let mut remaining = vec.to_vec();
            remaining.remove(i);
            prefix.push(item);
            permutations_helper(&remaining, n - 1, prefix, result);
            prefix.pop();
        }
    }

    let mut result = Vec::new();
    let mut prefix = Vec::new();
    permutations_helper(&vec, n, &mut prefix, &mut result);
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut weights = Vec::new();

    for _ in 0..n {
        weights.push(input.by_ref().take(n).collect::<Vec<usize>>());
    }

    let permutations = get_permutations((0..n).collect::<Vec<usize>>(), n);

    let mut min_w = usize::MAX;
    for permutation in permutations {
        let mut curr_w = 0;
        for start_idx in 0..n {
            let end_idx = (start_idx + 1) % n;
            curr_w += weights[permutation[start_idx]][permutation[end_idx]];
            if curr_w > min_w {
                break;
            }
        }
        if curr_w < min_w {
            min_w = curr_w;
        }
    }
    writeln!(output, "{}", min_w).unwrap();
    print!("{}", output);
    Ok(())
}
