// 그게 무슨 코드니...
#![allow(clippy::all)]
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim_end().chars().collect::<Vec<char>>();
    let n = input.len();
    let mut is_str = true;
    if n <= 2 {
        is_str = false;
    } else if input[0] == '"' && input[n - 1] == '"' {
        for idx in 1..(n - 1) {
            write!(output, "{}", input[idx]).unwrap();
            if input[idx] == '"' {
                is_str = false;
                break;
            }
        }
    } else {
        is_str = false;
    }

    if is_str {
        println!("{}", output);
    } else {
        println!("CE");
    }
    Ok(())
}
