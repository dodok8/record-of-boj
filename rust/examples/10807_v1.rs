//개수 세기

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for _ in 0..n {
        let k = input.next().unwrap();
        if let Some(val) = counts.get_mut(&k) {
            *val += 1;
        } else {
            counts.insert(k, 1);
        }
    }

    let ans = if let Some(val) = counts.get(&input.next().unwrap()) {
        val
    } else {
        &0_i32
    };
    writeln!(output, "{}", ans).unwrap();
    print!("{}", output);
    Ok(())
}
