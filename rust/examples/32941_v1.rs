// 왜 맘대로 예약하냐고

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let t = input.next().unwrap();
    let x = input.next().unwrap();

    let num_p = input.next().unwrap();

    let mut classes = vec![0; t + 1];
    for _ in 0..num_p {
        let k_i = input.next().unwrap();
        for __ in 0..k_i {
            classes[input.next().unwrap()] += 1;
        }
    }

    if classes[x] == num_p {
        writeln!(output, "YES")?;
    } else {
        writeln!(output, "NO")?;
    }
    print!("{}", output);
    Ok(())
}
