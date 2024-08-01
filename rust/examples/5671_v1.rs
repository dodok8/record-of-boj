// 호텔 방 번호

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn is_valid(num: usize) -> bool {
    let mut contained = [false; 10];
    let mut num = num;
    while num > 0 {
        let digit = num % 10;
        num /= 10;
        if contained[digit] {
            return false;
        } else {
            contained[digit] = true;
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    loop {
        let n = input.next();
        let m = input.next();

        if n.is_none() && m.is_none() {
            break;
        }
        let n = n.unwrap();
        let m = m.unwrap();

        let mut count = 0;

        (n..=m).for_each(|x| {
            if is_valid(x) {
                count += 1;
            }
        });
        writeln!(output, "{}", count).unwrap();
    }

    print!("{}", output);
    Ok(())
}
