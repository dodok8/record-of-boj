// 리모컨

use std::cmp::min;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let target = input.next().unwrap();
    let num_broken = input.next().unwrap();
    let mut is_broken = vec![false; 10];

    for num in input.take(num_broken) {
        is_broken[num] = true;
    }

    let mut min_count = usize::abs_diff(100, target);

    for num in 0..1000000_usize {
        let length = num.to_string().len();
        for (idx, digit) in num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .enumerate()
        {
            if is_broken[digit] {
                break;
            }

            if idx == length - 1 {
                min_count = min(min_count, usize::abs_diff(num, target) + length);
            }
        }
    }

    writeln!(output, "{}", min_count).unwrap();
    print!("{}", output);
    Ok(())
}
