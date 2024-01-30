// 소수인 팰린드롬
// 에라토스테네스의 체를 정확하게 짜보자.

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn is_palindrome(str: String) -> bool {
    let a = str.chars();
    let b = a.clone().rev();
    a.zip(b).take(str.len() / 2).all(|(x, y)| x == y)
}
trait Primes {
    fn get_primes_le(num: usize) -> Vec<usize>;
}

impl Primes for usize {
    /**
    curr이 현재 소수일 경우, 배수 체크는 curr * curr 부터 하는 최적화만 적용된 버전
    curr * some (some < curr)의 경우, some 이 합성수든 소수이던 간에 curr 보다 작은 소수의 배수이므로 이미 지워졌기 때문.
    2와 3의 배수를 제외하면, mod6에 대해 1,5인 수만 남는다.
    i번째 이 수에 해당하는 값은 `let curr = 6 * ((i+1) / 2) + 1 + (i + 1) % 2 * 4`
    이를 다시 idx로 돌리려면 `(j - 1) / 3 - 1`
    */
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let primes: Vec<usize> = usize::get_primes_le(b)
        .iter()
        .filter(|&&x| x >= a)
        .filter(|&&x| x <= b)
        .cloned()
        .collect();
    for prime in primes {
        if is_palindrome(prime.to_string()) {
            writeln!(output, "{}", prime).unwrap();
        }
    }
    writeln!(output, "-1").unwrap();
    print!("{}", output);
    Ok(())
}
