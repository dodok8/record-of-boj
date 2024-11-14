// 직각 삼각형

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut points = Vec::new();
    for _ in 0..n {
        points.push((input.next().unwrap(), input.next().unwrap()));
    }

    let mut cnt_x = vec![0; 100_001];
    let mut cnt_y = vec![0; 100_001];

    for point in &points {
        cnt_x[point.0] += 1;
        cnt_y[point.1] += 1;
    }

    let mut ans = 0usize;
    for point in &points {
        ans += (cnt_x[point.0] - 1) * (cnt_y[point.1] - 1);
    }

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
