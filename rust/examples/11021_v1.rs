use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    for idx in 1..(input.next().unwrap() + 1) {
        writeln!(
            output,
            "Case #{}: {}",
            idx,
            input.next().unwrap() + input.next().unwrap()
        )
        .unwrap();
    }
    print!("{}", output);
    Ok(())
}
