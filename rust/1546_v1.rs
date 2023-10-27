use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let test_count = input.next().unwrap() as usize;
    let mut scores: Vec<i32> = Vec::new();
    for _ in 0..test_count {
        scores.push(input.next().unwrap())
    }
    let max_value = *scores.iter().max().unwrap();
    let total_score: i32 = scores.iter().sum();
    writeln!(
        output,
        "{:.5}",
        total_score as f64 / max_value as f64 * 100.0 / test_count as f64
    )
    .unwrap();
    print!("{}", output);
    Ok(())
}
