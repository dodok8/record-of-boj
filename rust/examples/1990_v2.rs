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
    모든 소수는 2를 제외하면 홀수 이므로, 홀수에 대해서만 생각해도 된다.
    2*idx + 3의 소수 여부가 is_prime[idx] 이다.
    또한 홀수 + 홀수 = 짝수 이므로, curr * curr + 2*n*curr 이렇게 해서 홀수만 판정하도록 한다.
    1 도 홀수니
    */
    fn get_primes_le(num: usize) -> Vec<usize> {
        let mut is_prime = vec![true; num / 2 - 1];
        let mut primes = vec![2];

        for idx in 0..num / 2 - 1 {
            if is_prime[idx] {
                let curr = 2 * idx + 3;
                for next in (curr * curr..=num).step_by(curr * 2) {
                    let next_idx = (next - 3) / 2;
                    is_prime[next_idx] = false;
                }
                primes.push(curr);
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
