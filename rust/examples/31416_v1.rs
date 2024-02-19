// 가상 검증 기술

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let count_test = input.next().unwrap();
    for _ in 0..count_test {
        let t_a = input.next().unwrap();
        let t_b = input.next().unwrap();
        let v_a = input.next().unwrap();
        let v_b = input.next().unwrap();

        let end_b = t_b * v_b;

        if end_b >= t_a * v_a {
            writeln!(output, "{}", end_b).unwrap();
            continue;
        }
        let curr_a = end_b / t_a;
        let left_a = v_a - curr_a;
        if left_a % 2 == 0 {
            writeln!(output, "{}", end_b + t_a * left_a / 2).unwrap();
        } else {
            writeln!(output, "{}", end_b - (end_b % t_a) + t_a * (left_a + 1) / 2).unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
