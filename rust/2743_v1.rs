//단어 길이 재기
use std::error::Error;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.trim().len());
    Ok(())
}
