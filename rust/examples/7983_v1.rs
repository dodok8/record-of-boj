// 내일 할거야

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut assignments = Vec::new();
    for _ in 0..n {
        let duration = input.next().unwrap();
        let end = input.next().unwrap();
        assignments.push((duration, end));
    }

    assignments.sort_unstable_by_key(|(_x, y)| std::cmp::Reverse(*y));

    let mut vacation = assignments[0].1 - assignments[0].0;
    for (duration, end) in assignments.iter().skip(1) {
        if vacation >= *end {
            vacation = *end - *duration;
        } else {
            vacation -= *duration;
        }
    }
    writeln!(output, "{}", vacation).unwrap();
    print!("{}", output);
    Ok(())
}
