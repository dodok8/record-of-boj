// 소수 최소 공배수

trait Primes {
    fn get_primes_le(num: usize) -> Vec<usize>;
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
}

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let primes: HashSet<usize> = usize::get_primes_le(1000000).into_iter().collect();

    let _n = input.next().unwrap();
    let mut ans = 1;

    for num in input {
        if primes.contains(&num) && ans % num != 0 {
            ans *= num;
        }
    }

    if ans == 1 {
        writeln!(output, "-1")?;
    } else {
        writeln!(output, "{}", ans)?;
    }
    print!("{}", output);
    Ok(())
}
