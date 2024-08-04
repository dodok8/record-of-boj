// 배열 전체 탐색하기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let nums = input.by_ref().take(n).collect::<Vec<usize>>();

    let op_1 = |x: usize| nums.iter().filter(|&&num| num >= x).count();
    let op_2 = |x: usize| nums.iter().filter(|&&num| num > x).count();
    let op_3 = |x: usize, y: usize| {
        nums.iter()
            .filter(|&&num| num >= x)
            .filter(|&&num| num <= y)
            .count()
    };

    for _ in 0..m {
        match input.next().unwrap() {
            1 => writeln!(output, "{}", op_1(input.next().unwrap())).unwrap(),
            2 => writeln!(output, "{}", op_2(input.next().unwrap())).unwrap(),
            3 => writeln!(
                output,
                "{}",
                op_3(input.next().unwrap(), input.next().unwrap())
            )
            .unwrap(),
            _ => todo!(),
        }
    }

    print!("{}", output);
    Ok(())
}
