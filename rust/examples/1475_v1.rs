// 방 번호
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut needs = vec![0; 9];
    for letter in input
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
    {
        match letter {
            6 | 9 => needs[6] += 1,
            _ => needs[letter] += 1,
        }
    }
    needs[6] += needs[6] % 2;
    needs[6] /= 2;
    writeln!(output, "{}", needs.iter().max().unwrap()).unwrap();
    print!("{}", output);
    Ok(())
}
