// 블랙잭

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::usize;

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

    let _n = input.next().unwrap();
    let m = input.next().unwrap();

    let nums: Vec<usize> = input.collect();

    let mut ans = usize::MAX;
    for combi in get_combinations(nums, 3) {
        let sum: usize = combi.iter().sum();
        if ans.abs_diff(m) > sum.abs_diff(m) && sum <= m {
            ans = sum;
        }
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
