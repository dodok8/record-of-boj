// 십자카드 문제

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn split_digits(num: &usize) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut num = *num;

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits.reverse();
    digits
}

fn get_clock_num(num: &usize) -> usize {
    let digits = split_digits(num);
    (0..4)
        .map(|idx| {
            (0..4)
                .map(|jdx| digits[(idx + jdx) % 4] * 10usize.pow((3 - jdx) as u32))
                .sum()
        })
        .min()
        .unwrap()
}
fn check_clock_num(num: &usize) -> bool {
    get_clock_num(num) == *num
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let digits = input.take(4).collect::<Vec<usize>>();
    let target = get_clock_num(&digits.iter().enumerate().fold(0, |acc, (jdx, &digit)| {
        acc + digit * 10usize.pow((3 - jdx) as u32)
    }));

    let ans = (1111..=target).filter(|&idx| check_clock_num(&idx)).count();
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
