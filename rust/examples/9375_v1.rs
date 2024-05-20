// 패션왕 신해빈

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let num_test = input.next().unwrap().parse::<usize>()?;
    for _ in 0..num_test {
        let n = input.by_ref().next().unwrap().parse::<usize>()?;
        let mut combinations: HashMap<&str, usize> = HashMap::new();
        for __ in 0..n {
            let _v = input.next().unwrap();
            let key = input.next().unwrap();
            if let Some(val) = combinations.get_mut(key) {
                *val += 1;
            } else {
                combinations.insert(key, 2);
            }
        }
        let ans = combinations.values().product::<usize>() - 1;
        writeln!(output, "{}", ans).unwrap();
    }
    print!("{}", output);
    Ok(())
}
