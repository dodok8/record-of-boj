// 붙임성 좋은 총총이

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;

    let mut rainbow_dance: HashSet<&str> = HashSet::with_capacity(n);
    rainbow_dance.insert("ChongChong");
    for _ in 0..n {
        let first = input.next().unwrap();
        let second = input.next().unwrap();
        if rainbow_dance.contains(second) || rainbow_dance.contains(first) {
            rainbow_dance.insert(first);
            rainbow_dance.insert(second);
        }
    }
    writeln!(output, "{}", rainbow_dance.len()).unwrap();
    print!("{}", output);
    Ok(())
}
