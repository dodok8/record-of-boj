// 명령 프롬프트

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;

    let mut words: Vec<char> = input.next().unwrap().chars().collect();

    for _ in 0..(n - 1) {
        for (idx, letter) in input.next().unwrap().chars().enumerate() {
            if words[idx] != letter {
                words[idx] = '?';
            }
        }
    }

    writeln!(output, "{}", words.iter().collect::<String>())?;
    print!("{}", output);
    Ok(())
}
