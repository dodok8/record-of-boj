// 당직 근무표

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut counts = vec![0; n + 1];
    let mut possible = true;
    for _ in 0..n {
        let soldier = input.next().unwrap();
        counts[soldier] += 1;
        if counts[soldier] > (n + 1) / 2 {
            possible = false;
            break;
        }
    }
    if possible {
        writeln!(output, "YES").unwrap();
    } else {
        writeln!(output, "NO").unwrap();
    }
    print!("{}", output);
    Ok(())
}
