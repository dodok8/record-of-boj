// 암호 해독기

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn num_to_char(num: usize) -> char {
    match num {
        0 => ' ',
        1..=26 => (b'A' + (num - 1) as u8) as char,
        27..=52 => (b'a' + (num - 27) as u8) as char,
        _ => todo!(),
    }
}

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
    let mut letter_set: HashMap<char, i64> = HashMap::new();
    for code in codes {
        let letter = num_to_char(code);
        let curr_cnt = letter_set.get(&letter).unwrap_or_else(|| &0);
        letter_set.insert(letter, curr_cnt + 1);
    }

    let mut finish = true;
    for letter in letters {
        let curr_cnt = letter_set.get_mut(&letter);
        if let Some(&mut mut val) = curr_cnt {
            val -= 1;
            if val < 0 {
                finish = false;
                break;
            }
        } else {
            finish = false;
            break;
        }
    }
    if finish {
        writeln!(output, "y")?;
    } else {
        writeln!(output, "n")?;
    }
    print!("{}", output);
    Ok(())
}
