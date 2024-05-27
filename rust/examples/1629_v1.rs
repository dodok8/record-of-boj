// 곱셈

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_powers(b: usize, c: usize, powers: &mut Vec<usize>) -> usize {
    if powers[b] == 0 {
        if b % 2 == 0 {
            powers[b] = get_powers(b / 2, c, powers) * get_powers(b / 2, c, powers) % c;
        } else {
            powers[b] = get_powers(b / 2, c, powers)
                * get_powers(b / 2, c, powers)
                * get_powers(1, c, powers)
                % c;
        }
    }
    powers[b]
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();

    let mut powers = vec![0; b + 1];
    powers[0] = 1;
    powers[1] = a % c;
    writeln!(output, "{}", get_powers(b, c, &mut powers)).unwrap();
    print!("{}", output);
    Ok(())
}
