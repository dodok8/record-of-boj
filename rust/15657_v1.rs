.//N과 M (8)
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_non_desc_combinations(mut vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    fn non_desc_combinations_helper(
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
            let remaining = vec[i..].to_vec();
            prefix.push(item);
            non_desc_combinations_helper(&remaining, n - 1, prefix, result);
            prefix.pop();
        }
    }
    vec.sort_unstable();
    // 크기 순으로 정렬하면, 현재 원소와 같은 원소만 남겨두면 된다.
    let mut result = Vec::new();
    let mut prefix = Vec::new();
    non_desc_combinations_helper(&vec, n, &mut prefix, &mut result);
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

    let non_desc_combinations = get_non_desc_combinations(nums, m);
    for combination in non_desc_combinations {
        for num in combination {
            write!(output, "{} ", num).unwrap();
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
