// 카드2

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let n = input.next().unwrap();
    let mut cards: VecDeque<i32> = VecDeque::from((1..(n + 1)).collect::<Vec<i32>>());
    let mut idx = 1_usize;
    while cards.len() > 1 {
        if idx % 2 == 1 {
            cards.pop_front().unwrap();
        } else {
            let popped = cards.pop_front().unwrap();
            cards.push_back(popped);
        }
        idx += 1;
    }

    writeln!(output, "{}", cards.front().unwrap()).unwrap();
    print!("{}", output);
    Ok(())
}
