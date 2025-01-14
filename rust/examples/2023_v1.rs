// 신기한 소수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

trait Primes {
    fn get_primes_le(num: usize) -> Vec<bool>;
}
impl Primes for usize {
    fn get_primes_le(num: usize) -> Vec<bool> {
        let mut is_prime = vec![true; num / 3 + 1];
        let mut primes = vec![2, 3];
        let mut result = vec![false; num + 1];
        result[2] = true;
        result[3] = true;
        for idx in 0..is_prime.len() {
            let curr = 6 * ((idx + 1) / 2) + 1 + (idx + 1) % 2 * 4;
            if is_prime[idx] {
                primes.push(curr);
                if curr < result.len() {
                    result[curr] = true;
                }
            }
            for &min_p in primes[2..].iter() {
                let v = curr * min_p;
                if v > num {
                    break;
                }
                let v_idx = (v - 1) / 3 - 1;
                is_prime[v_idx] = false;
                if curr % min_p == 0 {
                    break;
                }
            }
        }
        result
    }
}

fn search(curr_num: usize, depth: usize, n: usize, result: &mut Vec<usize>, primes: &Vec<bool>) {
    if depth == n {
        result.push(curr_num)
    } else {
        for idx in [1, 3, 5, 7, 9] {
            let next_num = 10 * curr_num + idx;
            if primes[next_num] {
                search(next_num, depth + 1, n, result, primes);
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let primes = usize::get_primes_le(10_usize.pow(n as u32) + 1);

    let mut results = Vec::new();

    for start in [2, 3, 5, 7] {
        search(start, 1, n, &mut results, &primes);
    }

    for result in results {
        writeln!(output, "{}", result)?;
    }
    print!("{}", output);
    Ok(())
}
