// 여러분의 다리가 되어 드리겠습니다!

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut parents: Vec<Option<usize>> = vec![None; n + 1];
    for _ in 1..=(n - 2) {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        // println!("{} {}", a, b);
        if a < b {
            if let Some(val) = parents[a] {
                parents[b] = Some(val);
            } else {
                parents[b] = Some(a);
            }
        } else if let Some(val) = parents[b] {
            parents[a] = Some(val);
        } else {
            parents[a] = Some(b);
        }
    }

    for (idx, num) in parents.iter().enumerate().skip(1) {
        if let Some(_val) = num {
            continue;
        } else {
            write!(output, "{} ", idx).unwrap();
        }
    }
    
    println!("{}", output);
    Ok(())
}
