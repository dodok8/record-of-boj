// 암호 해독기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("\n");
    let _n = input.next().unwrap().parse::<usize>()?;
    let codes = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>)
        .collect::<Vec<usize>>();
    let letters = input.next().unwrap().chars();

    let mut count_space = 0;
    let mut count_lower = 0;
    let mut count_upper = 0;

    for code in codes {
        match code {
            0 => count_space += 1,
            1..=26 => count_upper += 1,
            27..=52 => count_lower += 1,
            _ => panic!(),
        }
    }

    for letter in letters {
        if letter == ' ' {
            count_space -= 1;
        } else if letter.is_ascii_lowercase() {
            count_lower -= 1;
        } else {
            count_upper -= 1;
        }
    }

    if count_space == 0 && count_lower == 0 && count_upper == 0 {
        writeln!(output, "y").unwrap();
    } else {
        writeln!(output, "n").unwrap();
    }
    print!("{}", output);
    Ok(())
}
