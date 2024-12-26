// The Merchant of Venice

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let k = input.next().unwrap();

    for idx in 1..=k {
        let n = input.next().unwrap();
        let s = input.next().unwrap();
        let d = input.next().unwrap();

        let limit = s * d;

        let mut ans = 0;

        for _ in 0..n {
            let dist = input.next().unwrap();
            let v = input.next().unwrap();
            if dist <= limit {
                ans += v;
            }
        }

        writeln!(output, "Data Set {}:", idx)?;
        if idx != k {
            writeln!(output, "{}\n", ans)?;
        } else {
            writeln!(output, "{}", ans)?;
        }
    }
    print!("{}", output);
    Ok(())
}
