// 선 긋기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap();

    let mut points = Vec::new();

    for _ in 0..(n as usize) {
        points.push((input.next().unwrap(), input.next().unwrap()));
    }

    points.sort_unstable();
    let mut lines = vec![points[0]];

    let mut idx = 0;
    for point in points {
        if lines[idx].0 <= point.0 && point.0 <= lines[idx].1 {
            if point.1 >= lines[idx].1 {
                lines[idx].1 = point.1;
            }
        } else {
            lines.push(point);
            idx += 1;
        }
    }

    let result: i64 = lines.iter().map(|(x, y)| y - x).sum();
    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
