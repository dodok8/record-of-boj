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
    let t = input.next().unwrap();

    let dists: Vec<f64> = input.by_ref().take(n).collect();
    let vels: Vec<f64> = input.take(n).collect();

    let ranges = dists
        .iter()
        .zip(vels)
        .map(|(&x, v)| (x - v * t, x + v * t))
        .collect::<Vec<(f64, f64)>>();

    let mut start = f64::MIN;
    let mut end = f64::MAX;

    for &range in ranges.iter() {
        let rounded_start = (range.0 * 10_000.0).round() / 10_000.0;
        let rounded_end = (range.1 * 10_000.0).round() / 10_000.0;

        start = start.max(rounded_start);
        end = end.min(rounded_end);
    }

    if start <= end {
        writeln!(output, "1")?;
    } else {
        writeln!(output, "0")?;
    }
    print!("{}", output);
    Ok(())
}
