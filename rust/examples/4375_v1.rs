//1

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    for n in input {
        let mut a = 1;
        let mut sum = a;
        let mut ans = 1;

        while sum % n != 0 {
            a = a * 10 % n;
            sum = (sum + a) % n;
            ans += 1;
        }

        writeln!(output, "{}", ans)?;
    }
    print!("{}", output);
    Ok(())
}
