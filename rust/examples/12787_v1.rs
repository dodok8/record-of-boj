// 지금 밥이 문제냐

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn ipv8_to_u64(ip: &str) -> u64 {
    ip.split('.')
        .map(|octet| octet.parse::<u64>().unwrap())
        .fold(0, |acc, octet| (acc << 8) | octet)
}

fn u64_to_ipv8(num: u64) -> String {
    (0..8)
        .map(|i| ((num >> (8 * (7 - i))) & 255).to_string())
        .collect::<Vec<String>>()
        .join(".")
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>()?;

    for _ in 0..n {
        let m = input.next().unwrap().parse::<usize>()?;
        match m {
            1 => writeln!(output, "{}", ipv8_to_u64(input.next().unwrap()))?,
            2 => writeln!(
                output,
                "{}",
                u64_to_ipv8(input.next().unwrap().parse::<u64>()?)
            )?,
            _ => panic!("Invalid Input"),
        }
    }

    print!("{}", output);
    Ok(())
}
