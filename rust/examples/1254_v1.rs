// 팰린드롬 만들기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let word = input.next().unwrap().chars().collect::<Vec<char>>();
    let n = word.len();
    for offset in 0..n {
        let mut is_pal = true;
        for (idx, &letter) in word.iter().rev().enumerate() {
            if offset + idx >= n {
                break;
            }
            if letter != word[offset + idx] {
                is_pal = false;
                break;
            }
        }
        if is_pal {
            writeln!(output, "{}", n + offset).unwrap();
            break;
        }
    }
    print!("{}", output);
    Ok(())
}
