//치킨 배달
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn combinations(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
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

fn get_distance(first: (usize, usize), second: (usize, usize)) -> usize {
    first.0.abs_diff(second.0) + first.1.abs_diff(second.1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let limit_chicken = input.next().unwrap();
    let mut houses = vec![];
    let mut chickens = vec![];

    for idx in 0..n {
        for jdx in 0..n {
            let curr_value = input.next().unwrap();
            if curr_value == 1 {
                houses.push((idx, jdx));
            } else if curr_value == 2 {
                chickens.push((idx, jdx));
            }
        }
    }
    let chicken_combinations = combinations((0..chickens.len()).collect(), limit_chicken);

    let mut smallest_chicken_distance = usize::MAX;
    for chicken_combination in chicken_combinations {
        let mut curr_chicken_distance = 0_usize;
        for house in &houses {
            curr_chicken_distance += chicken_combination
                .iter()
                .map(|chicken_idx| get_distance(chickens[*chicken_idx], *house))
                .min()
                .unwrap();
        }
        if curr_chicken_distance < smallest_chicken_distance {
            smallest_chicken_distance = curr_chicken_distance;
        }
    }
    writeln!(output, "{}", smallest_chicken_distance).unwrap();
    print!("{}", output);
    Ok(())
}
