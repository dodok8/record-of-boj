// 비밀번호 찾기

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>()?;
    let m = input.next().unwrap().parse::<usize>()?;

    let mut passwords: HashMap<String, &str> = HashMap::new();

    for _ in 0..n {
        let k = input.next().unwrap().to_string();
        let v = input.next().unwrap();
        passwords.insert(k, v);
    }
    for _ in 0..m {
        let k = input.next().unwrap().to_string();
        writeln!(output, "{}", passwords.get(&k).unwrap()).unwrap();
    }
    print!("{}", output);
    Ok(())
}
