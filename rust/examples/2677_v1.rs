// 에드삭 만들기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn number_to_bin(int: &u64, negative: bool) -> [usize; 17] {
    let mut result = [0; 17];
    let int = *int / 5u64.pow(16u32);

    for idx in 0..17 {
        result[16 - idx] = ((int >> idx) & 1) as usize;
    }

    if negative {
        for idx in 0..17 {
            result[idx] = 1 - result[idx];
        }

        for idx in (0..17).rev() {
            if result[idx] == 0 {
                result[idx] = 1;
                break;
            } else {
                result[idx] = 0;
            }
        }
    }

    result
}

fn convert_to_string(digits: [usize; 17]) -> String {
    let mut output = String::new();
    let codes = [
        'P', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'J', '#', 'S', 'Z', 'K', '*', '?', 'F',
        '@', 'D', '!', 'H', 'N', 'M', '&', 'L', 'X', 'G', 'A', 'B', 'C', 'V',
    ];

    let letter_idx = u64::from_str_radix(
        &digits[0..5]
            .iter()
            .map(|d| d.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap() as usize;
    let number = u64::from_str_radix(
        &digits[5..16]
            .iter()
            .map(|d| d.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    let word = if u64::from_str_radix(
        &digits[16..]
            .iter()
            .map(|d| d.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap()
        == 1
    {
        "D"
    } else {
        "F"
    };
    write!(output, "{} {} {}", codes[letter_idx], number, word).unwrap();
    output
}

fn command_to_number(command_10: &[u64]) -> Result<u64, &'static str> {
    let mut number: u64 = 0;

    for (idx, &digit) in command_10.iter().enumerate() {
        let multiplier = 10_u64
            .checked_pow((command_10.len() - 1 - idx) as u32)
            .ok_or("Overflow in calculating multiplier")?;
        let addend = digit
            .checked_mul(multiplier)
            .ok_or("Overflow in calculating addend")?;

        number = number
            .checked_add(addend)
            .ok_or("Overflow in adding to number")?;
    }

    Ok(number)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    for _ in 0..n {
        let r = input.next().unwrap();
        let raws = r.chars().collect::<Vec<char>>();
        let negative = raws[0] == '-';
        let mut command_10 = [0_u64; 17];

        if negative {
            let mut idx = 0;
            for &raw in raws.iter().skip(1) {
                if raw == '.' {
                    continue;
                } else {
                    command_10[idx] = raw.to_digit(10).unwrap() as u64;
                    idx += 1;
                }
            }
        } else {
            let mut idx = 0;
            for &raw in raws.iter() {
                if raw == '.' {
                    continue;
                } else {
                    command_10[idx] = raw.to_digit(10).unwrap() as u64;
                    idx += 1;
                }
            }
        }

        if let Ok(number) = command_to_number(&command_10) {
            if negative && number > 10000000000000000 {
                writeln!(output, "INVALID VALUE")?;
            } else if number >= 10000000000000000 && !negative {
                writeln!(output, "INVALID VALUE")?;
            } else {
                writeln!(
                    output,
                    "{} ",
                    convert_to_string(number_to_bin(&number, negative)),
                )?;
            }
        } else {
            writeln!(output, "INVALID VALUE")?;
        }
    }
    print!("{}", output);
    Ok(())
}
