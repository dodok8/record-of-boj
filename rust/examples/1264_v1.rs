// 모음의 개수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut count = 0_usize;
    for letter in input.chars() {
        let letter = letter.to_ascii_lowercase();
        match letter {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count += 1;
            }
            '\n' => {
                writeln!(output, "{}", count).unwrap();
                count = 0;
            }
            '#' => {
                break;
            }
            _ => {}
        }
    }
    print!("{}", output);
    Ok(())
}
