// 곱셈을 누가 이렇게 해 ㅋㅋ


use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn strange_convert(a: &str, b: &str) -> u128 {
    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();
    let mut result = VecDeque::new();

    for (&a, &b) in a
        .iter()
        .rev()
        .zip(b.iter().rev().chain(std::iter::repeat(&'1')))
    {
        result.push_front(a.to_digit(10).unwrap() * b.to_digit(10).unwrap());
    }

    let result: Vec<u32> = result.into();
    let result = result.iter().map(|num| num.to_string()).collect::<String>();

    result.parse::<u128>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let t = input.next().unwrap().parse::<u128>()?;

    for _ in 0..t {
        let a = input.next().unwrap();
        let b = input.next().unwrap();

        let mut strange_mul = 0;

        if a.len() > b.len() {
            strange_mul = strange_convert(a, b);
        } else {
            strange_mul = strange_convert(b, a)
        }

        let a: u128 = a.parse()?;
        let b: u128 = b.parse()?;

        if strange_mul == a * b {
            writeln!(output, "1")?;
        } else {
            writeln!(output, "0")?;
        }
    }
    print!("{}", output);
    Ok(())
}
