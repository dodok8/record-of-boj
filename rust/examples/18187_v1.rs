// 평면 분할

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut lines = [0, 0, 0];
    let mut ans = 1;

    for idx in 1..=n {
        ans += 1 + lines[0] + lines[1] + lines[2] - lines[idx % 3];
        lines[idx % 3] += 1;
    }

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
