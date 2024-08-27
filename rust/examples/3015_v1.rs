// 오아시스 재결합
// 오사시스 재결합 기념

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let nums: Vec<usize> = input.take(n).collect();

    let mut ans = 0;
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();

    for &num in &nums {
        let mut cnt = 1;
        while !stack.is_empty() && stack.back().unwrap().0 <= num {
            if stack.back().unwrap().0 == num {
                let (_, d_cnt) = stack.pop_back().unwrap();
                ans += d_cnt;
                cnt = d_cnt + 1;
            } else {
                ans += stack.pop_back().unwrap().1;
            }
        }

        if !stack.is_empty() {
            ans += 1;
        }
        stack.push_back((num, cnt));
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
