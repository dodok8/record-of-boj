// 신기한 소수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as usize;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn search(curr_num: usize, depth: usize, n: usize, result: &mut Vec<usize>) {
    if depth == n {
        result.push(curr_num);
        return;
    }
    
    for digit in [1, 3, 5, 7, 9] {
        let next_num = curr_num * 10 + digit;
        if is_prime(next_num) {
            search(next_num, depth + 1, n, result);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let n = input.trim().parse::<usize>()?;
    
    let mut results = Vec::new();
    for start in [2, 3, 5, 7] {
        if is_prime(start) {
            search(start, 1, n, &mut results);
        }
    }
    
    results.sort();
    for result in results {
        writeln!(output, "{}", result)?;
    }
    print!("{}", output);
    Ok(())
}