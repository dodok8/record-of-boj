// 돌림판 문자열

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    let source = input.next().unwrap().chars().collect::<Vec<char>>();
    let _m = input.next().unwrap().parse::<usize>()?;
    let target = input.next().unwrap().chars().collect::<Vec<char>>();

    let mut count = 0;
    let mut s_idx = 0usize;
    for t in target {
        let mut c_idx = s_idx;
        let mut is_continue = true;
        while t != source[c_idx] {
            c_idx += 1;
            c_idx %= n;
            count += 1;
            if c_idx == s_idx {
                is_continue = false;
                break;
            }
        }
        if !is_continue {
            count = -1;
            break;
        } else {
            s_idx = (c_idx + 1) % n;
            count += 1;
        }
    }
    writeln!(output, "{}", count).unwrap();
    print!("{}", output);
    Ok(())
}
