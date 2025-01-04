// 헤일스톤 수열

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let t = input.next().unwrap();

    for _ in 0..t {
        let mut n = input.next().unwrap();
        let mut max_n = n;
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = n * 3 + 1
            }

            if max_n < n {
                max_n = n;
            }
        }
        writeln!(output, "{}", max_n)?;
    }
    print!("{}", output);
    Ok(())
}
