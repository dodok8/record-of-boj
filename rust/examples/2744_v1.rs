// 대소문자 바꾸기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    for char in input.chars() {
        if char.is_uppercase() {
            write!(output, "{}", char.to_lowercase()).unwrap();
        } else {
            write!(output, "{}", char.to_uppercase()).unwrap();
        }
    }
    println!("{}", output);
    Ok(())
}
