// 양갈래 배열 출력 하기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
#[allow(unused_mut)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    let m = input.next().unwrap().parse::<usize>()?;

    let mut max;
    let mut idx;
    let mut jdx;

    let start_dir: usize = match input.next().unwrap() {
        "U" => {
            idx = 0;
            jdx = m / 2;
            max = (n * m + n) / 2;
            0
        }
        "D" => {
            idx = n - 1;
            jdx = m / 2;
            max = (n * m + n) / 2;
            2
        }
        "R" => {
            idx = n / 2;
            jdx = m - 1;
            max = (n * m + m) / 2;
            3
        }
        "L" => {
            idx = n / 2;
            jdx = 0;
            max = (n * m + m) / 2;
            1
        }
        &_ => unreachable!(),
    };
    let mut curr_dir = start_dir;
    let mut curr_num = 1;
    let mut twin = vec![vec![0; m]; n];
    let mut is_mirror = false;
    loop {
        twin[idx][jdx] = curr_num;
        if is_mirror {
            match start_dir {
                2 | 0 => twin[idx][2 * (m / 2) - jdx] = curr_num,
                1 | 3 => twin[2 * (n / 2) - idx][jdx] = curr_num,
                _ => unreachable!(),
            }
        }
        if curr_num == max + 1 {
            break;
        }
        match curr_dir {
            0 => {
                if idx < n - 1 && twin[idx + 1][jdx] == 0 {
                    idx += 1;
                } else {
                    curr_dir = (curr_dir + 1) % 4;
                    jdx += 1;
                    is_mirror = true;
                }
            }
            2 => {
                if 0 < idx && twin[idx - 1][jdx] == 0 {
                    idx -= 1;
                } else {
                    curr_dir = (curr_dir + 1) % 4;
                    jdx -= 1;
                    is_mirror = true;
                }
            }
            1 => {
                if jdx < m - 1 && twin[idx][jdx + 1] == 0 {
                    jdx += 1;
                } else {
                    curr_dir = (curr_dir + 1) % 4;
                    idx -= 1;
                    is_mirror = true;
                }
            }
            3 => {
                if 0 < jdx && twin[idx][jdx - 1] == 0 {
                    jdx -= 1;
                } else {
                    curr_dir = (curr_dir + 1) % 4;
                    idx += 1;
                    is_mirror = true;
                }
            }
            _ => unreachable!(),
        }
        curr_num += 1;
    }

    for row in twin {
        for col in row {
            write!(output, "{} ", col)?;
        }
        writeln!(output)?;
    }

    print!("{}", output);
    Ok(())
}
