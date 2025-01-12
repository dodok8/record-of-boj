// 소수 경로

use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
trait Primes {
    fn get_primes_le(num: usize) -> HashSet<usize>;
}

impl Primes for usize {
    fn get_primes_le(num: usize) -> HashSet<usize> {
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
        primes.iter().copied().collect()
    }
}

fn get_digit_changes(num: &usize, primes: &HashSet<usize>, distances: &[usize]) -> HashSet<usize> {
    let mut result = HashSet::new();
    for idx in 0..4 {
        let multiplier = 10_usize.pow(idx);
        for jdx in (if idx == 3 { 1 } else { 0 })..10 {
            let num_changed =
                num / (multiplier * 10) * (multiplier * 10) + jdx * multiplier + num % multiplier;
            if num_changed >= 1000
                && num_changed != *num
                && distances[num_changed] == usize::MAX
                && primes.contains(&num_changed)
            {
                result.insert(num_changed);
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let primes = usize::get_primes_le(10_000);
    let t = input.next().unwrap();
    for _ in 0..t {
        let num_start = input.next().unwrap();
        let num_end = input.next().unwrap();

        let mut distances = [usize::MAX; 10_000];
        let mut travel_q = VecDeque::new();
        travel_q.push_back((0_usize, num_start));
        distances[num_start] = 0;

        // println!("{:?}", get_digit_changes(&2, &primes, &distances));
        while !travel_q.is_empty() {
            let (dist, num) = travel_q.pop_front().unwrap();

            for next_num in get_digit_changes(&num, &primes, &distances) {
                distances[next_num] = dist + 1;
                travel_q.push_back((distances[next_num], next_num));
            }
        }

        if distances[num_end] == usize::MAX {
            writeln!(output, "Impossible")?;
        } else {
            writeln!(output, "{}", distances[num_end])?;
        }
    }

    print!("{}", output);
    Ok(())
}
