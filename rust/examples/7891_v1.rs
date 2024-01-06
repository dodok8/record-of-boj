use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i128>);
    for _ in 0..input.next().unwrap() {
        let [a, b] = [0; 2].map(|_| input.next().unwrap());
        writeln!(output, "{}", a + b).unwrap();
    }
    print!("{}", output);
    Ok(())
}
