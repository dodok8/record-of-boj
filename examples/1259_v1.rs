// 펠린드롬수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn is_palindrome(word: &str) -> bool {
    let reversed_word = word.chars().rev().collect::<String>();

    word.eq(&reversed_word)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    loop {
        let num = input.next().unwrap().to_string();
        if num.eq("0") {
            break;
        }
        if is_palindrome(&num) {
            writeln!(output, "yes").unwrap();
        } else {
            writeln!(output, "no").unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
