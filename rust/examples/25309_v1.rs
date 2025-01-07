// K 개의 소수

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

    let n = input.next().unwrap();
    let k = input.next().unwrap();

    // 기본 검사
    if n < 2 * k || (k > n) {
        writeln!(output, "-1")?;
        print!("{}", output);
        return Ok(());
    }

    let primes = usize::get_primes_le(n + 10);
    let mut ans = Vec::new();
    // k=1인 경우
    if k == 1 {
        if primes[n] {
            writeln!(output, "{}", n)?;
        } else {
            writeln!(output, "-1")?;
        }

        print!("{}", output);
        return Ok(());
    } else if k == 2 {
        if n < 4 {
            writeln!(output, "-1")?;
            print!("{}", output);
            return Ok(());
        } else if n % 2 == 1 {
            if primes[n - 2] {
                ans.extend([2, n - 2].iter())
            } else {
                writeln!(output, "-1")?;
                print!("{}", output);
                return Ok(());
            }
        } else {
            ans.extend(get_goldbach_pair(n, &primes));
        }
    } else {
        // k > 2
        if n < k * 2 {
            writeln!(output, "-1")?;
            print!("{}", output);
            return Ok(());
        } else if n % 2 == 0 {
            ans.extend(vec![2; k - 2]);
            ans.extend(get_goldbach_pair(n - (k - 2) * 2, &primes));
        } else {
            ans.push(3);
            ans.extend(vec![2; k - 3]);
            ans.extend(get_goldbach_pair(n - 2 * k + 3, &primes));
        }
    }

    for num in ans {
        write!(output, "{} ", num)?;
    }
    writeln!(output)?;

    print!("{}", output);
    Ok(())
}
