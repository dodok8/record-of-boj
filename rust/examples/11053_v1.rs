// 가장 긴 증가하는 부분 수열
// O(n^2) 풀이
use std::cmp::max;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut data = Vec::new();
    for _ in 0..n {
        data.push(input.next().unwrap());
    }
    let mut len_lis = vec![0_usize; n];
    let mut result = 0;
    for idx in 0..n {
        for jdx in 0..idx {
            if data[idx] > data[jdx] {
                len_lis[idx] = max(len_lis[idx], len_lis[jdx] + 1);
            }
            if result < len_lis[idx] {
                result = len_lis[idx];
            }
        }
    }
    writeln!(output, "{}", result + 1).unwrap();
    print!("{}", output);
    Ok(())
}
