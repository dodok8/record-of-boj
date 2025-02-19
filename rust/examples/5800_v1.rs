// 성적 통계

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    for idx in 1..=input.next().unwrap() {
        let n = input.next().unwrap();
        let mut scores: Vec<_> = input.by_ref().take(n).collect();
        scores.sort_unstable();
        writeln!(output, "Class {}", idx)?;
        writeln!(
            output,
            "Max {}, Min {}, Largest gap {}",
            scores.last().unwrap(),
            scores.first().unwrap(),
            scores.windows(2).map(|w| w[1] - w[0]).max().unwrap()
        )?;
    }
    print!("{}", output);
    Ok(())
}
