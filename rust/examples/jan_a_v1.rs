use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let _n = input.next().unwrap().parse::<i32>().unwrap();
    let k = input.next().unwrap().parse::<i32>().unwrap();
    let mut point = (0, 0);
    let letters: Vec<char> = input.next().unwrap().chars().collect();

    for _ in 0..k {
        for letter in &letters {
            match letter {
                'U' => point = (point.0 + 1, point.1),
                'R' => point = (point.0, point.1 + 1),
                'L' => point = (point.0, point.1 - 1),
                'D' => point = (point.0 - 1, point.1),
                _ => todo!(),
            }

            if point.0 == 0 && point.1 == 0 {
                break;
            }
        }
    }
    if point.0 == 0 && point.1 == 0 {
        writeln!(output, "YES").unwrap();
    } else {
        writeln!(output, "NO").unwrap();
    }
    print!("{}", output);
    Ok(())
}
