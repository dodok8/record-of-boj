//최댓값

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut max_idx = 0;
    let mut max_val = 0;
    for idx in 0..9 {
        let val = input.next().unwrap();
        if val > max_val {
            max_idx = idx;
            max_val = val;
        }
    }
    writeln!(output, "{}", max_val).unwrap();
    writeln!(output, "{}", max_idx + 1).unwrap();
    print!("{}", output);
    Ok(())
}
