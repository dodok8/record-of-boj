// 2023은 무엇이 특별할까?
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();

        if (n + 1) % (n % 100) == 0 {
            writeln!(output, "Good").unwrap();
        } else {
            writeln!(output, "Bye").unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
