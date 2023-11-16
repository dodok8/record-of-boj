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
    let m = input.next().unwrap();
    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(input.next().unwrap())
    }
    nums.sort_unstable();
    let permutations = get_permutations(nums, m);
    for permutation in permutations {
        for num in permutation {
            write!(output, "{} ", num).unwrap();
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
