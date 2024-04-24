// 찾기
// 지문부터가 KMP 설명이다

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

trait Kmp {
    fn get_indexes(&self, target: &str) -> Vec<usize>;
}

impl Kmp for String {
    fn get_indexes(&self, target: &str) -> Vec<usize> {
        fn get_partial_match(target: &str) -> Vec<usize> {
            let target = target.as_bytes();
            let len = target.len();
            let mut pi = vec![0; len];
            let (mut begin, mut matched) = (1_usize, 0_usize);
            while begin + matched < len {
                if target[begin + matched] == target[matched] {
                    matched += 1;
                    pi[begin + matched - 1] = matched;
                } else if matched == 0 {
                    begin += 1;
                } else {
                    begin += matched - pi[matched - 1];
                    matched = pi[matched - 1];
                }
            }
            pi
        }

        let mut results = Vec::new();
        let source = self.as_bytes();
        let pi = get_partial_match(target);
        let target = target.as_bytes();

        if source.len() < target.len() {
            return results;
        }
        let (mut begin, mut matched) = (0_usize, 0_usize);
        while begin <= source.len() - target.len() {
            if matched < target.len() && source[begin + matched] == target[matched] {
                matched += 1;

                if matched == target.len() {
                    results.push(begin);
                }
            } else if matched == 0 {
                begin += 1;
            } else {
                begin += matched - pi[matched - 1];
                matched = pi[matched - 1];
            }
        }
        results
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split('\n');

    let source = input.next().unwrap().to_string();
    let target = input.next().unwrap();

    let results = source.get_indexes(target);
    writeln!(output, "{}", results.len()).unwrap();
    if !results.is_empty() {
        for num in results {
            write!(output, "{} ", num + 1).unwrap();
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
