// 이변마작

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    let mut mazac: HashMap<&str, usize> = HashMap::new();

    let mut ans = 0;
    for idx in 1..=n {
        let key = input.next().unwrap();
        let count = mazac.entry(key).or_insert(0);
        *count += 1;
        if *count == 5 {
            ans = idx;
            break;
        }
    }

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
