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

    points.sort_unstable();

    let mut compressed_points = HashMap::new();

    for (idx, &point) in points.iter().enumerate() {
        compressed_points.insert(point, idx);
    }

    let mut compressed_ranges = vec![0; 2 * n + 1];
    for (first, second) in ranges {
        let &start = compressed_points.get(&first).unwrap();
        let &end = compressed_points.get(&second).unwrap();

        compressed_ranges[start] += 1;
        compressed_ranges[end + 1] -= 1;
    }

    for idx in 1..=2 * n {
        compressed_ranges[idx] += compressed_ranges[idx - 1];
    }

    // 자기 자신은 포함에서 제외해야 하므로 1을 뺀다.
    writeln!(output, "{}", compressed_ranges.iter().max().unwrap() - 1).unwrap();
    print!("{}", output);
    Ok(())
}
