// 은?행 털!자 1

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap();

    let mut plans: HashMap<i64, i64> = HashMap::new();

    for _ in 0..n {
        let x = input.next().unwrap();
        let t = input.next().unwrap();
        let c = input.next().unwrap();

        plans.entry(t - x).and_modify(|v| *v += c).or_insert(c);
    }
    writeln!(output, "{}", plans.into_values().max().unwrap())?;
    print!("{}", output);
    Ok(())
}
