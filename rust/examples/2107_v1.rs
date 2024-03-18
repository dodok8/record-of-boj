// 포함하는 구간

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut ranges = Vec::new();
    let mut points = Vec::new();

    for _ in 0..n {
        let first = input.next().unwrap();
        let second = input.next().unwrap();
        ranges.push((first, second));
        points.push(first);
        points.push(second);
    }

    // 정렬 후, 정렬 순서에 따라 좌표 압축 진행.
    points.sort_unstable();

    let mut compressed_points = HashMap::new();

    for (idx, &point) in points.iter().enumerate() {
        compressed_points.insert(point, idx);
    }

    let mut compressed_ranges = Vec::new();

    for (first, second) in ranges {
        let &start = compressed_points.get(&first).unwrap();
        let &end = compressed_points.get(&second).unwrap();
        compressed_ranges.push((start, end));
    }

    let mut counts = vec![0; compressed_ranges.len()];

    for idx in 0..compressed_ranges.len() {
        for jdx in 0..idx {
            let r1 = compressed_ranges[idx];
            let r2 = compressed_ranges[jdx];

            if r1.0 <= r2.0 && r1.1 >= r2.1 {
                counts[idx] += 1;
            }
            if r2.0 <= r1.0 && r2.1 >= r1.1 {
                counts[jdx] += 1;
            }
        }
    }
    writeln!(output, "{}", counts.iter().max().unwrap()).unwrap();
    print!("{}", output);
    Ok(())
}
