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
    i번째 이 수에 해당하는 값은 6 * (i/2) + 1 + (i%2)*4  이다. / 는 몫 연산이다. 주의!
    idx = 0, 1, 2
    a = 0,0,1,1,2,2,3,3, ...
    b = 1,5,1,5,1,5,1,5, ...
    6a+b = 1,5,7,11,13,17,19,23, ...
    그리고 이 값에서 되돌리는 방법은 1을 빼고 3으로 나누는 거다.
    */
    fn get_primes_le(num: usize) -> Vec<usize> {
        let mut is_prime = vec![true; num / 3 + 1];
        let mut primes = vec![2, 3];

        for idx in 1..is_prime.len() {
            if is_prime[idx] {
                let curr = 6 * (idx / 2) + 1 + (idx % 2) * 4;
                for next in (curr * curr..=num).step_by(curr) {
                    if next % 6 == 1 || next % 6 == 5 {
                        let next_idx = (next - 1) / 3;
                        is_prime[next_idx] = false;
                    }
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
