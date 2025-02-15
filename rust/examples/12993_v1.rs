// 이동3

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn convert_base_3(num: &usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut num = *num;
    while num > 0 {
        result.push(num % 3);
        num /= 3;
    }

    result.reverse();

    result
}

fn check_digit_sum_one(x: usize, y: usize) -> bool {
    let x_base3 = convert_base_3(&x);
    let y_base3 = convert_base_3(&y);

    let max_len = x_base3.len().max(y_base3.len());

    x_base3
        .into_iter()
        .rev()
        .chain(std::iter::repeat(0))
        .zip(y_base3.into_iter().rev().chain(std::iter::repeat(0)))
        .take(max_len)
        .all(|(d1, d2)| d1 + d2 == 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let x = input.next().unwrap();
    let y = input.next().unwrap();

    if check_digit_sum_one(x, y) {
        writeln!(output, "1")?;
    } else {
        writeln!(output, "0")?;
    }
    print!("{}", output);
    Ok(())
}
