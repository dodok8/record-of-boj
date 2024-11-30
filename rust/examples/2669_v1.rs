// 직사각형 네개의 합집합의 면적 구하기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut areas = vec![vec![0; 101]; 101];
    let mut ans = 0;

    for _ in 0..4 {
        let x1 = input.next().unwrap();
        let y1 = input.next().unwrap();
        let x2 = input.next().unwrap();
        let y2 = input.next().unwrap();

        for idx in x1..x2 {
            for jdx in y1..y2 {
                if areas[idx][jdx] == 0 {
                    areas[idx][jdx] = 1;
                }
            }
        }
    }

    for row in &areas {
        for &cell in row {
            if cell == 1 {
                ans += 1;
            }
        }
    }

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
