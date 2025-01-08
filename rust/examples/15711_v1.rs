// 환상의 짝궁

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

const SIEVE_MAX: usize = 1_000_000;

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
            // p 가 sqrt(n)보다 크다는 소리이므로 break
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
    let small_primes: HashSet<usize> = usize::get_primes_le(SIEVE_MAX).into_iter().collect();

    for _ in 0..t {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        let sum = a + b;

        if sum <= 3 {
            // 합이 3 보다 작으면 불가능
            writeln!(output, "NO")?;
        } else if sum % 2 == 0 {
            // 골드바흐의 추측
            writeln!(output, "YES")?;
        } else if is_prime_large(sum - 2, &small_primes) {
            // 합이 홀수인데 소수 + 소수 조합이려면 하나는 무조건 2이고, 다른 하나는 소수 여야함.
            writeln!(output, "YES")?;
        } else {
            writeln!(output, "NO")?;
        }
    }

    print!("{}", output);
    Ok(())
}
