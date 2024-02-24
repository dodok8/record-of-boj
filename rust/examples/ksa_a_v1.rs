// 소수가 아닌 수 3

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::iter::FromIterator;

fn generate_numbers(digits: Vec<usize>) -> Vec<usize> {
    let mut results = digits.iter().map(|&d| d.to_string()).collect::<Vec<_>>();

    for _ in 0..digits.len() - 1 {
        let mut new_results = Vec::new();
        for res in &results {
            for &digit in &digits {
                new_results.push(format!("{}{}", res, digit));
            }
        }
        results.extend(new_results);
    }

    let mut results: Vec<usize> = results
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    results.sort_unstable();
    results
}

trait Primes {
    fn get_primes_le(num: usize) -> Vec<usize>;
}

impl Primes for usize {
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
    let n = input.next().unwrap();
    let mut digits = Vec::new();
    for _ in 0..n {
        digits.push(input.next().unwrap());
    }
    let nums = generate_numbers(digits);
    let primes: HashSet<usize> =
        HashSet::from_iter(usize::get_primes_le(nums[nums.len() - 1]).iter().cloned());
    let mut result = true;
    let mut ans = 0;
    for num in nums {
        if !primes.contains(&num) {
            result = false;
            ans = num;
            break;
        }
    }
    if result {
        writeln!(output, "NO").unwrap();
    } else {
        writeln!(output, "YES").unwrap();
        writeln!(output, "{}", ans).unwrap();
    }
    print!("{}", output);
    Ok(())
}
