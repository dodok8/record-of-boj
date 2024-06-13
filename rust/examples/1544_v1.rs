// 사이클 단어

fn rotate_string(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    let last = chars.remove(0);
    chars.push(last);
    chars.into_iter().collect()
}

fn all_rotations(s: &str) -> Vec<String> {
    let mut result = vec![s.to_string()];
    let mut current = s.to_string();

    for _ in 0..s.len() - 1 {
        current = rotate_string(&current);
        result.push(current.clone());
    }

    result
}

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    let mut check = HashSet::new();
    let mut count = 0;
    for _ in 0..n {
        let word = input.next().unwrap();
        if !check.contains(word) {
            count += 1;
            check.extend(all_rotations(word));
        }
    }
    writeln!(output, "{}", count).unwrap();
    print!("{}", output);
    Ok(())
}
