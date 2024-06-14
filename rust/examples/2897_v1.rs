// 몬스터 트럭

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn count_parking(parking: &[Vec<char>]) -> Vec<usize> {

    let r = parking.len();
    let c = parking[0].len();

    let mut result = vec![0; r * c + 1];

    for idx in 0..(r - 1) {
        for jdx in 0..(c - 1) {
            let mut crushed_car = 0;
            let mut available = true;
            for (dx, dy) in [(0, 1), (1, 0), (1, 1), (0, 0)] {
                match parking[idx + dx][jdx + dy] {
                    '#' => {
                        available = false;
                        break;
                    }
                    'X' => {
                        crushed_car += 1;
                    }
                    _ => continue,
                }
            }
            if available {
                result[crushed_car] += 1;
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let num_r = input.next().unwrap().parse::<usize>()?;
    let _num_c = input.next().unwrap().parse::<usize>()?;

    let mut parking = Vec::new();

    for _ in 0..num_r {
        parking.push(input.next().unwrap().chars().collect::<Vec<char>>());
    }
    for limit in count_parking(&parking).iter().take(5) {
        writeln!(output, "{}", limit).unwrap();
    }
    print!("{}", output);
    Ok(())
}
