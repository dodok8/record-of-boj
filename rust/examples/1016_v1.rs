// 제곱 ㄴㄴ 수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let min = input.next().unwrap();
    let max = input.next().unwrap();

    let mut is_nn = vec![false; max - min + 1];
    let mut ans = max - min + 1;

    let mut idx = 2;
    while idx * idx <= max {
        let mut jdx = if min % (idx * idx) == 0 {
            min / (idx * idx)
        } else {
            min / (idx * idx) + 1
        };

        while jdx * idx * idx <= max {
            if !is_nn[jdx * idx * idx - min] {
                is_nn[jdx * idx * idx - min] = true;
                ans -= 1;
            }
            jdx += 1;
        }
        idx += 1;
    }

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
