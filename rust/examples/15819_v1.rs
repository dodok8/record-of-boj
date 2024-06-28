// 너의 핸들은

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let _ = input.next().unwrap();
    let idx = input.next().unwrap().parse::<usize>()? - 1;
    let mut handles = input.collect::<Vec<&str>>();
    handles.sort_unstable();
    writeln!(output, "{}", handles[idx]).unwrap();
    print!("{}", output);
    Ok(())
}
