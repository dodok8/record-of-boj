// 삼각 초콜릿 포장(Sweet)

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;

    let mut passed = vec![vec![false; n]; n];
    let mut chocos = vec![vec![None; n]; n];
    for (idx, letters) in input.enumerate() {
        for (jdx, letter) in letters.chars().enumerate() {
            chocos[idx][jdx] = Some(letter);
        }
    }
    let mut result = 1;

    for idx in 0..n {
        for jdx in 0..=idx {
            if passed[idx][jdx] {
                continue;
            }
            // println!("{} {}", idx, jdx);
            passed[idx][jdx] = true;
            match chocos[idx][jdx] {
                Some('R') => {
                    if jdx > idx + 1 {
                        break;
                    }
                    if idx == n - 1 {
                        result = 0;
                        break;
                    }
                    let next_1 = chocos[idx + 1][jdx].unwrap();
                    let next_2 = chocos[idx + 1][jdx + 1].unwrap();
                    if next_1 == 'R' && next_2 == 'R' {
                        // println!("True: ({},{}) ({},{})", idx + 1, jdx, idx + 1, jdx + 1);
                        passed[idx + 1][jdx] = true;
                        passed[idx + 1][jdx + 1] = true;
                    } else {
                        result = 0;
                        break;
                    }
                }
                Some('B') => {
                    if jdx + 1 > idx {
                        break;
                    }
                    if idx == n - 1 {
                        result = 0;
                        break;
                    }
                    let next_1 = chocos[idx][jdx + 1].unwrap();
                    let next_2 = chocos[idx + 1][jdx + 1].unwrap();
                    if next_1 == 'B' && next_2 == 'B' {
                        // println!("True: ({},{}) ({},{})", idx, jdx + 1, idx + 1, jdx + 1);
                        passed[idx][jdx + 1] = true;
                        passed[idx + 1][jdx + 1] = true;
                    } else {
                        result = 0;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
