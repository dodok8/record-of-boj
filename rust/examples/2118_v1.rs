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
    let mut distances = vec![0_usize; n + 1];
    for idx in 1..=n {
        let curr = input.next().unwrap();
        distances[idx] = distances[idx - 1] + curr;
    }

    let mut ans = 0;
    let mut idx = 0;
    let mut jdx = idx + 1;

    while idx < jdx && jdx < n + 1 {
        let right = distances[jdx] - distances[idx];
        let left = distances[n] - distances[jdx] + distances[idx];
        if right < left {
            jdx += 1;
        } else {
            idx += 1;
        }
        ans = max(ans, min(left, right));
    }

    writeln!(output, "{}", ans).unwrap();
    print!("{}", output);
    Ok(())
}
