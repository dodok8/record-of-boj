// 두 개의 탑

use std::cmp::{max, min};
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut distances = vec![0_usize];
    for idx in 1..=n {
        let curr = input.next().unwrap();
        distances.push(distances[idx - 1] + curr);
    }

    let mut ans = 0;
    for idx in 1..(n / 2 + 1) {
        if idx + 1 > n {
            break;
        }
        for jdx in idx + 1..n {
            ans = max(
                ans,
                min(
                    distances[jdx] - distances[idx],
                    distances[n] - distances[jdx] + distances[idx],
                ),
            );
        }
    }

    writeln!(output, "{}", ans).unwrap();
    print!("{}", output);
    Ok(())
}
