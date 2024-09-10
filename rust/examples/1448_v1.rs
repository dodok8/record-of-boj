// 삼각형 만들기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap() as usize;
    let mut ans = -1;

    let mut straws = input.collect::<Vec<i64>>();
    straws.sort_unstable_by(|a, b| b.cmp(a));

    for idx in 0..(n - 2) {
        if straws[idx] < straws[idx + 1] + straws[idx + 2] {
            ans = straws[idx] + straws[idx + 1] + straws[idx + 2];
            break;
        }
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
