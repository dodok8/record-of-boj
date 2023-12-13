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
    let mut count_cases = vec![0; k + 1];
    count_cases[0] = 1;
    let mut coins = Vec::new();
    for _ in 0..n {
        coins.push(input.next().unwrap());
    }

    for coin in &coins {
        for idx in *coin..(k + 1) {
            count_cases[idx] += count_cases[idx - coin];
        }
    }
    writeln!(output, "{}", count_cases[k]).unwrap();
    print!("{}", output);
    Ok(())
}
