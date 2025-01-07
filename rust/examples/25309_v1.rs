// K 개의 소수

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

fn get_goldbach_pair(n: usize, primes: &Vec<usize>, primes_set: &HashSet<usize>) -> Vec<usize> {
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
    let k = input.next().unwrap();

    // 기본 검사
    if n < 2 * k || (k > n) {
        writeln!(output, "-1")?;
        print!("{}", output);
        return Ok(());
    }

    let primes = usize::get_primes_le(n);
    let primes_set: HashSet<usize> = HashSet::from_iter(primes.iter().cloned());
    let mut ans = Vec::with_capacity(k);

    // k=1인 경우
    if k == 1 {
        if primes_set.contains(&n) {
            writeln!(output, "{}", n)?;
        } else {
            writeln!(output, "-1")?;
        }
        print!("{}", output);
        return Ok(());
    }

    // k=2인 경우
    if k == 2 {
        let mut found = false;
        for &prime in &primes {
            if prime > n {
                break;
            }
            if primes_set.contains(&(n - prime)) {
                ans.push(prime);
                ans.push(n - prime);
                found = true;
                break;
            }
        }
        if !found {
            writeln!(output, "-1")?;
        } else {
            for num in ans {
                write!(output, "{} ", num)?;
            }
            writeln!(output)?;
        }
        print!("{}", output);
        return Ok(());
    }

    // k≥3인 경우
    let first = if n % 2 == 0 { 2 } else { 3 };
    ans.push(first);
    
    // 남은 수를 (k-1)개로 분해
    let remaining = n - first;
    if remaining >= 2 * (k - 1) {
        let second = if (remaining - 2 * (k - 2)) >= 4 { 2 } else { 3 };
        ans.push(second);
        
        // k-2개의 2 추가
        ans.extend(vec![2; k - 2]);
        
        // 마지막으로 남은 수 처리
        let final_remaining = remaining - second - 2 * (k - 2);
        if final_remaining < 2 {
            writeln!(output, "-1")?;
            print!("{}", output);
            return Ok(());
        }
        
        // final_remaining이 소수인지 확인
        if !primes_set.contains(&final_remaining) {
            writeln!(output, "-1")?;
            print!("{}", output);
            return Ok(());
        }
        ans.pop();  // 마지막 2를 제거
        ans.push(final_remaining);
        
        for num in ans {
            write!(output, "{} ", num)?;
        }
        writeln!(output)?;
    } else {
        writeln!(output, "-1")?;
    }
    
    print!("{}", output);
    Ok(())
}