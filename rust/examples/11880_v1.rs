// 개미

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    for _ in 0..input.next().unwrap() {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        let c = input.next().unwrap();

        writeln!(
            output,
            "{}",
            [
                a.pow(2) + (b + c).pow(2),
                b.pow(2) + (a + c).pow(2),
                c.pow(2) + (a + b).pow(2)
            ]
            .iter()
            .min()
            .unwrap()
        )?;
    }

    print!("{}", output);
    Ok(())
}
