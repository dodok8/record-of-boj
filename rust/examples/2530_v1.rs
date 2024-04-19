// 인공지능 시계

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut time = input.next().unwrap() * 3600;
    time += input.next().unwrap() * 60;
    time += input.next().unwrap();
    time += input.next().unwrap();

    let hour = time / 3600;
    time = time % 3600;
    let min = time / 60;
    time = time % 60;
    let second = time;
    writeln!(output, "{} {} {}", hour % 24, min, second).unwrap();
    print!("{}", output);
    Ok(())
}
