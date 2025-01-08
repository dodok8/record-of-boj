// 환상의 짝궁

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

fn get_goldbach_pair(n: usize, primes: &Vec<bool>) -> Vec<usize> {
    let mut result = Vec::new();

    for idx in 0..=n {
        if primes[idx] && primes[n - idx] {
            result.push(idx);
            result.push(n - idx);
            break;
        }
    }

    result
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let t = input.next().unwrap();
    let mut cases: Vec<usize> = Vec::with_capacity(t);
    for _ in 0..t {
        cases.push(input.next().unwrap() + input.next().unwrap());
    }

    let &max_prime = cases.iter().max().unwrap();
    let primes = usize::get_primes_le(max_prime);

    for case in cases {
        if get_goldbach_pair(case, &primes).len() != 2 {
            writeln!(output, "NO")?;
        } else {
            writeln!(output, "YES")?;
        }
    }

    print!("{}", output);
    Ok(())
}
