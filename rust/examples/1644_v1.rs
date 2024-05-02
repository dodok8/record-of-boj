// 소수의 연속합

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut result = 0;
    let primes = usize::get_primes_le(n);

    let mut start = 0_usize;
    let mut end = 0_usize;
    let mut curr = 0;

    while end < primes.len() {
        if curr >= n {
            if curr == n {
                result += 1;
            }
            curr -= primes[start];
            start += 1;
        } else {
            curr += primes[end];
            end += 1;
        }
    }

    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
