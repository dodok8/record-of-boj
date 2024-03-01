// !!초콜릿 중독 주의!!

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;

    for _ in 0..n {
        let equation = input.next().unwrap().chars().collect::<Vec<char>>();
        let start = if equation[equation.len() - 1] == '1' || equation[equation.len() - 1] == '!' {
            1_usize
        } else {
            0_usize
        };
        let mut count = 0;
        for letter in equation {
            if letter != '!' {
                break;
            }
            count += 1;
        }

        if count % 2 == 0 {
            writeln!(output, "{}", start).unwrap();
        } else {
            writeln!(output, "{}", 1 - start).unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
