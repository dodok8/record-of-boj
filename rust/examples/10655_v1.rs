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
    let sum = {
        let mut sum = 0;
        for idx in 0..(n - 1) {
            sum += get_distance(distances[idx], distances[idx + 1]);
        }
        sum
    };

    let mut result = sum;

    for idx in 1..(n - 1) {
        let curr_d = sum
            - (get_distance(distances[idx], distances[idx + 1])
                + get_distance(distances[idx], distances[idx - 1]))
            + get_distance(distances[idx - 1], distances[idx + 1]);

        if curr_d < result {
            result = curr_d;
        }
    }
    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
