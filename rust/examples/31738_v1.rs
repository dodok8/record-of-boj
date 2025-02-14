// 매우 어려운 문제

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();

    if n >= m {
        writeln!(output, "0")?;
        print!("{}", output);
        return Ok(());
    }

    let ans = {
        let mut facto = 1_usize;
        let mut idx = n;

        loop {
            facto *= idx;
            facto %= m;
            if facto == 0 || idx == 1 {
                break facto;
            }
            idx -= 1;
        }
    };
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
