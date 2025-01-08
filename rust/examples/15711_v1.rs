// 환상의 짝궁

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

const SIEVE_MAX: usize = 1_000_000; // 에라토스테네스의 체를 적용할 최대값

fn get_primes_le(num: usize) -> HashSet<usize> {
    let num = num.min(SIEVE_MAX) as usize;
    let mut is_prime = vec![true; num / 3 + 1];
    let mut primes = vec![2, 3];

    for idx in 0..is_prime.len() {
        let curr = 6 * ((idx + 1) / 2) + 1 + (idx + 1) % 2 * 4;
        if curr > num {
            break;
        }
        if is_prime[idx] {
            primes.push(curr);
        }
        for &min_p in primes[2..].iter() {
            let v = curr * min_p;
            if v > num {
                break;
            }
            let v_idx = (v - 1) / 3 - 1;
            if v_idx < is_prime.len() {
                is_prime[v_idx] = false;
            }
            if curr % min_p == 0 {
                break;
            }
        }
    }
    primes.into_iter().collect()
}

fn is_prime_large(n: usize, small_primes: &HashSet<usize>) -> bool {
    if n <= SIEVE_MAX {
        return small_primes.contains(&n);
    }

    if n <= 1 {
        return false;
    }

    // 작은 소수들로 먼저 나누어보기
    for &p in small_primes {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            return false;
        }
    }

    let sqrt_n = (n as f64).sqrt() as usize;
    let mut i = SIEVE_MAX;
    while i <= sqrt_n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let t = input.next().unwrap();
    let small_primes = get_primes_le(SIEVE_MAX);

    for _ in 0..t {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        let sum = a + b;

        if sum <= 3 {
            writeln!(output, "NO")?;
        } else if sum % 2 == 0 {
            writeln!(output, "YES")?;
        } else if is_prime_large(sum - 2, &small_primes) {
            writeln!(output, "YES")?;
        } else {
            writeln!(output, "NO")?;
        }
    }

    print!("{}", output);
    Ok(())
}
