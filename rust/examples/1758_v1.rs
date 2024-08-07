// 알바생 강호

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut tips = input.take(n).collect::<Vec<usize>>();
    tips.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = 0;

    for (idx, &tip) in tips.iter().enumerate() {
        ans += tip.saturating_sub(idx);
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
