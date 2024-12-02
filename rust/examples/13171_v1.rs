// A

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let a = input.next().unwrap();
    let x = input.next().unwrap();
    let mut squares = vec![1; 64];
    squares[0] = a % 1_000_000_007;

    for idx in 1..64 {
        squares[idx] = squares[idx - 1].pow(2) % 1_000_000_007;
    }

    // println!("{:?}", squares);

    let mut ans = 1;

    for (idx, bit) in (0..usize::BITS).map(|n| (x >> n) & 1).enumerate() {
        // println!("{} {} {}", idx, bit, bit * squares[idx] % 1_000_000_007);
        if bit != 0 {
            ans = (ans * squares[idx]) % 1_000_000_007;
        }
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
