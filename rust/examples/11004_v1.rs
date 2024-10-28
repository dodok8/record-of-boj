// K 번째 수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let _n = input.next().unwrap();
    let k = input.next().unwrap() as usize;

    let mut nums = input.collect::<Vec<i64>>();

    nums.sort_unstable();
    writeln!(output, "{}", nums[k - 1])?;
    print!("{}", output);
    Ok(())
}
