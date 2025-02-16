// 오리와 박수치는 춘배

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let k = input.next().unwrap();

    let mut count = 0usize;
    let mut last_clap = 0;

    for _ in 0..n {
        let t = input.next().unwrap();

        if !(t <= last_clap && last_clap < (t + k)) {
            last_clap = t + k;
            count += 1;
        }
    }

    writeln!(output, "{}", count)?;
    print!("{}", output);
    Ok(())
}
