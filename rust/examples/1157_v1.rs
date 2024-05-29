// 단어 공부

use std::cmp::Ordering;
use std::error::Error;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let word = input.next().unwrap().chars();
    let mut counts = [0_usize; 26];
    let char_to_num = |c: char| ((c as u32) - ('A' as u32)) as usize;
    let num_to_char = |n: u32| -> char { ((n + ('A' as u32)) as u8) as char };

    for letter in word {
        counts[char_to_num(letter.to_ascii_uppercase())] += 1;
    }

    let mut curr_max = 0;
    let mut max_idx = 0;
    let mut is_only = true;

    for (idx, &count) in counts.iter().enumerate() {
        match count.cmp(&curr_max) {
            Ordering::Greater => {
                max_idx = idx;
                curr_max = count;
                is_only = true;
            }
            Ordering::Equal => {
                is_only = false;
            }
            _ => {}
        }
    }

    if is_only {
        println!("{}", num_to_char(max_idx as u32));
    } else {
        println!("?");
    }
    Ok(())
}
