// 동전2

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let k = input.next().unwrap();
    let mut coins = Vec::new();
    for _ in 0..n {
        coins.push(input.next().unwrap());
    }

    let mut cache = vec![usize::MAX; k + 1];
    cache[0] = 0;
    for coin in coins {
        for idx in coin..=k {
            cache[idx] = (cache[idx - coin] + 1).min(cache[idx]);
        }
    }
    writeln!(output, "{}", cache[k]).unwrap();
    print!("{}", output);
    Ok(())
}
