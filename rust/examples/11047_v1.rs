// 동전0

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let _n = input.next().unwrap();
    let goal = input.next().unwrap();

    let coins = input.rev().collect::<Vec<usize>>();

    let mut curr = goal;
    let mut num_coin = 0;
    for coin in coins {
        num_coin += curr / coin;
        curr %= coin;
    }
    writeln!(output, "{}", num_coin).unwrap();
    print!("{}", output);
    Ok(())
}
