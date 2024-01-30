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
    */
    fn get_primes_le(num: usize) -> Vec<usize> {
        let mut is_prime = vec![true; num + 1];

        for curr in 2..num {
            if is_prime[curr] {
                for next in (curr * curr..=num).step_by(curr) {
                    is_prime[next] = false;
                }
            }
        }

        is_prime
            .iter()
            .enumerate()
            .skip(2)
            .filter_map(|(idx, &val)| val.then_some(idx))
            .collect()
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
