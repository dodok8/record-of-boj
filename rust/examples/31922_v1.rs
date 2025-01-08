// 이 대회는 이제 제 겁니다.

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>);
    
    let a = input.next().unwrap();
    let p = input.next().unwrap();
    let c = input.next().unwrap();
    
    let ans = std::cmp::max(a + c, p);
    
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}