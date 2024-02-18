// 님 게임

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let num = input.next().unwrap();
    let mut stones = Vec::new();
    let mut count_1 = 0;
    let mut some_idx = 0;
    for idx in 0..num {
        let stone = input.next().unwrap();
        if stone == 1 {
            count_1 += 1;
        } else {
            some_idx = idx;
        }
        stones.push(stone);
    }
    let mut xor_sum = 0;
    if count_1 == 0 || count_1 % 2 == 1 {
        xor_sum = stones.iter().fold(0, |sum, &stone| sum ^ stone);
    } else {
        xor_sum = stones.iter().fold(0, |sum, &stone| sum ^ stone);
        stones[some_idx] = 1;
    }
    if xor_sum == 0 {
        writeln!(output, "{}", "cubelover")?;
    } else {
        writeln!(output, "{}", "koosaga")?;
    }

    print!("{}", output);

    Ok(())
}
