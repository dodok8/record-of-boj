use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut original_cards = vec![0_usize; n];

    let mut previous_idx = VecDeque::from((0..n).collect::<Vec<usize>>());

    for idx in (1..=n).rev() {
        match input.next().unwrap() {
            1 => {
                let curr = previous_idx.pop_front().unwrap();
                original_cards[curr] = idx
            }
            2 => {
                let (first, second) = (
                    previous_idx.pop_front().unwrap(),
                    previous_idx.pop_front().unwrap(),
                );
                previous_idx.push_front(first);
                original_cards[second] = idx
            }
            3 => {
                let curr = previous_idx.pop_back().unwrap();
                original_cards[curr] = idx
            }
            _ => panic!(),
        }
    }

    for card in original_cards {
        write!(output, "{} ", card).unwrap();
    }
    println!("{}", output);
    Ok(())
}
