// 착신 전환 소동

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num = input.next().unwrap();
    let mut locked = HashSet::new();
    let mut nums = vec![0; num + 1];

    for idx in 1..(num + 1) {
        let lock = input.next().unwrap();
        nums[idx] = lock;
        if idx == lock {
            locked.insert(idx);
        }
    }

    let mut count = 0;
    for idx in locked.clone().into_iter() {
        count += 1;
        for jdx in 1..(num + 1) {
            if jdx == idx {
                continue;
            }
            nums[idx] = jdx;
            break;
        }
    }
    writeln!(output, "{}", count).unwrap();
    for num in nums.iter().skip(1) {
        write!(output, "{} ", num).unwrap();
    }
    println!("{}", output);
    Ok(())
}
