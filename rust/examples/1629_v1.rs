// 곱셈

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_powers(a: u128, b: u128, c: u128, powers: &mut HashMap<u128, u128>) -> u128 {
    if powers.get(&b).is_none() {
        if b % 2 == 0 {
            let temp = get_powers(a, b / 2, c, powers);
            powers.insert(b, (temp * temp) % c);
        } else {
            let temp = get_powers(a, b / 2, c, powers);
            powers.insert(b, (temp * temp) * a % c);
        }
    }
    *powers.get(&b).unwrap()
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u128>);
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();

    let mut powers = HashMap::new();
    powers.insert(0, 1);
    powers.insert(1, a % c);
    writeln!(output, "{}", get_powers(a, b, c, &mut powers)).unwrap();
    print!("{}", output);
    Ok(())
}
