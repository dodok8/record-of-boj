// 엘리스 트랙 매칭

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let num_friend: usize = str::parse(input.next().unwrap())?;
    let mut tracks = HashMap::new();
    tracks.insert("C", 0);
    tracks.insert("S", 0);
    tracks.insert("I", 0);
    tracks.insert("A", 0);
    for _ in 0..num_friend {
        let k = input.next().unwrap();
        *tracks.get_mut(k).unwrap() += 1;
    }
    let k = input.next().unwrap();
    writeln!(output, "{}", *tracks.get(k).unwrap())?;
    print!("{}", output);
    Ok(())
}
