// 알파벳 거리

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn char_to_num(c: &char) -> usize {
    ((*c as u32) - ('A' as u32)) as usize
}

fn get_char_distance(a: &char, b: &char) -> usize {
    let a = char_to_num(a);
    let b = char_to_num(b);
    if a >= b {
        a - b
    } else {
        a + 26 - b
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    for _ in 0..n {
        write!(output, "Distances:")?;
        let a_s = input.next().unwrap().chars();
        let b_s = input.next().unwrap().chars();
        for (a, b) in a_s.zip(b_s) {
            write!(output, " {}", get_char_distance(&b, &a))?;
        }
        writeln!(output)?;
    }

    print!("{}", output);
    Ok(())
}
