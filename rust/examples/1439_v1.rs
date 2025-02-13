// 뒤집기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn count_separated_groups(s: &str) -> Vec<i32> {
    if s.is_empty() {
        return vec![0, 0];
    }

    let chars: Vec<char> = s.chars().collect();
    let mut zeros = 0;
    let mut ones = 0;

    // 첫 번째 문자 처리
    if chars[0] == '0' {
        zeros = 1;
    } else {
        ones = 1;
    }

    // 나머지 문자들 처리
    for i in 1..chars.len() {
        if chars[i] != chars[i - 1] {
            if chars[i] == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
    }

    vec![zeros, ones]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let given = input.next().unwrap();
    writeln!(
        output,
        "{}",
        count_separated_groups(given).iter().min().unwrap()
    )?;
    print!("{}", output);
    Ok(())
}
