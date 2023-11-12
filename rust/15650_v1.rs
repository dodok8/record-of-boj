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
            // 이 부분만 현재 원소 지우는 걸로 하면 permutation 이겠구만
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
    let m = input.next().unwrap();
    let combinations = get_combinations((1..n + 1).collect(), m);
    for combination in combinations {
        for num in combination {
            write!(output, "{} ", num).unwrap();
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
