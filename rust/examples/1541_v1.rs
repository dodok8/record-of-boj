// 잃어버린 괄호

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split('-');

    let get_sum = |c: &str| c.split('+').flat_map(str::parse::<i32>).sum::<i32>();
    let mut ans: i32 = get_sum(input.next().unwrap());

    for num in input {
        ans -= get_sum(num);
    }
    writeln!(output, "{}", ans).unwrap();
    print!("{}", output);
    Ok(())
}
