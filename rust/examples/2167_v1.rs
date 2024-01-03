use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let m = input.next().unwrap() as usize;
    let mut prefix_sum = vec![vec![0; m]; n];

    for idx in 0..n {
        for jdx in 0..m {
            prefix_sum[idx][jdx] = input.next().unwrap();
        }
    }

    for idx in 0..n {
        for jdx in 1..m {
            prefix_sum[idx][jdx] += prefix_sum[idx][jdx - 1];
        }
    }

    for jdx in 0..m {
        for idx in 1..n {
            prefix_sum[idx][jdx] += prefix_sum[idx - 1][jdx];
        }
    }

    for _ in 0..input.next().unwrap() {
        let i = input.next().unwrap() as usize - 1;
        let j = input.next().unwrap() as usize - 1;
        let x = input.next().unwrap() as usize - 1;
        let y = input.next().unwrap() as usize - 1;

        let mut result = prefix_sum[x][y];

        if i >= 1 {
            result -= prefix_sum[i - 1][y];
        }
        if j >= 1 {
            result -= prefix_sum[x][j - 1];
        }
        if i >= 1 && j >= 1 {
            result += prefix_sum[i - 1][j - 1];
        }
        writeln!(output, "{}", result).unwrap();
    }

    print!("{}", output);
    Ok(())
}
