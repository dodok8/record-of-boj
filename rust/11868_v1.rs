use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse::<i32>)
        .flatten();
    let n = input.next().unwrap();
    let num_stones: Vec<i32> = input.by_ref().take(n as usize).collect();
    let ans = num_stones
        .into_iter()
        .reduce(|acc, curr| curr ^ acc)
        .unwrap();
    if ans == 0 {
        writeln!(output, "cubelover").unwrap();
    } else {
        writeln!(output, "koosaga").unwrap();
    }
    print!("{}", output);
    Ok(())
}
