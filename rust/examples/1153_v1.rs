// 네 개의 소수

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::iter::FromIterator;

trait Primes {
    fn get_primes_le(num: usize) -> Vec<usize>;
    fn get_primes_ge_le(min: usize, max: usize) -> Vec<usize>;
    fn get_n_primes(n: usize) -> Vec<usize>;
}
impl Primes for usize {
    fn get_primes_le(num: usize) -> Vec<usize> {
        let mut is_prime = vec![true; num / 3 + 1];
        let mut primes = vec![2, 3];
        for idx in 0..is_prime.len() {
            let curr = 6 * ((idx + 1) / 2) + 1 + (idx + 1) % 2 * 4;
            if is_prime[idx] {
                primes.push(curr);
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
        primes
    }
    fn get_primes_ge_le(min: usize, max: usize) -> Vec<usize> {
        usize::get_primes_le(max)
            .iter()
            .filter(|&&x| x >= min)
            .cloned()
            .collect()
    }
    fn get_n_primes(n: usize) -> Vec<usize> {
        if n == 2 {
            return vec![2, 3];
        }
        if n == 1 {
            return vec![2];
        }
        let mut is_prime = vec![true; n * 1000 / 3 + 1];
        let mut primes = vec![2, 3];
        let mut cnt = 2;
        for idx in 0..is_prime.len() {
            let curr = 6 * ((idx + 1) / 2) + 1 + (idx + 1) % 2 * 4;
            if is_prime[idx] {
                primes.push(curr);
                cnt += 1;
                if cnt == n {
                    return primes;
                }
            }
            for &min_p in primes[2..].iter() {
                let v = curr * min_p;
                let v_idx = (v - 1) / 3 - 1;
                if v_idx >= is_prime.len() {
                    break;
                }
                is_prime[v_idx] = false;
                if curr % min_p == 0 {
                    break;
                }
            }
        }
        primes
    }
}

fn split_two_primes(n: usize, primes: &Vec<usize>, primes_set: &HashSet<usize>) -> Vec<usize> {
    let mut result = Vec::new();

    for &prime in primes {
        if primes_set.contains(&(n - prime)) {
            result.push(prime);
            result.push(n - prime);
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
    let n = input.next().unwrap();
    let primes = usize::get_primes_le(n);
    let primes_set: HashSet<usize> = HashSet::from_iter(primes.iter().cloned());

    let mut ans: Vec<usize> = Vec::new();

    if n < 8 {
    } else if n % 2 == 0 {
        ans.extend([2, 2].iter());
        ans.extend(split_two_primes(n - 4, &primes, &primes_set));
    } else {
        ans.extend([2, 3].iter());
        ans.extend(split_two_primes(n - 5, &primes, &primes_set));
    }

    if ans.is_empty() {
        writeln!(output, "-1")?;
    } else {
        for num in ans {
            write!(output, "{} ", num)?;
        }
        writeln!(output)?;
    }
    print!("{}", output);
    Ok(())
}
