// 준오는 급식충이야!!

use core::f64;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let n = input.next().unwrap() as usize;
    let t = (input.next().unwrap() * 10000.0) as i64;

    let dists: Vec<i64> = input
        .by_ref()
        .take(n)
        .map(|n| (n * 10000.0) as i64)
        .collect();
    let vels: Vec<i64> = input.take(n).map(|n| (n * 10000.0) as i64).collect();

    let ranges = dists
        .iter()
        .zip(vels)
        .map(|(&x, v)| (x - v * t, x + v * t))
        .collect::<Vec<(i64, i64)>>();

    let mut start = i64::MIN;
    let mut end = i64::MAX;

    for &range in ranges.iter() {
        start = start.max(range.0);
        end = end.min(range.1);
    }

    if start <= end {
        writeln!(output, "1")?;
    } else {
        writeln!(output, "0")?;
    }
    print!("{}", output);
    Ok(())
}
