// 자작나무가 없소~

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    let name = input.next().unwrap();
    let mut cnt = 0;
    for _ in 0..n {
        let item = input.next().unwrap().to_string();
        let num = input.next().unwrap().parse::<usize>()?;
        if item.len() == name.len() && item.contains(name) {
            cnt += num;
        }
        if item.contains(&format!("_{}", name)) || item.contains(&format!("{}_", name)) {
            // println!("{}", item);
            cnt += num;
        }
    }

    writeln!(output, "{}", cnt).unwrap();
    print!("{}", output);
    Ok(())
}
