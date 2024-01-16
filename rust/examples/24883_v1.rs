// 자동완성
use std::error::Error;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    match input.next().unwrap() {
        "n" | "N" => println!("Naver D2"),
        _ => println!("Naver Whale"),
    }
    Ok(())
}
