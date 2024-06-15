// 마라톤 1

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_distance(s: (i32, i32), o: (i32, i32)) -> u32 {
    s.1.abs_diff(o.1) + s.0.abs_diff(o.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let n = input.next().unwrap() as usize;
    let mut distances = Vec::new();
    for _ in 0..n {
        distances.push((input.next().unwrap(), input.next().unwrap()));
    }
    let mut result = vec![u32::MAX; n];
    for skip in 1..(n - 1) {
        let mut previous = distances[0];
        let mut curr_d = 0;
        for idx in 1..n {
            if idx == skip {
                continue;
            }
            curr_d += get_distance(previous, distances[idx]);
            previous = distances[idx];
        }
        result[skip] = curr_d;
    }
    writeln!(output, "{}", result.iter().min().unwrap()).unwrap();
    print!("{}", output);
    Ok(())
}
