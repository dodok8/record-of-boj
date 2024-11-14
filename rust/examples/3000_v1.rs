// 직각 삼각형

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut points = Vec::new();
    for _ in 0..n {
        points.push((input.next().unwrap(), input.next().unwrap()));
    }

    let mut ans = 0;
    for (idx, point) in points.iter().enumerate() {
        let mut cnt_x = 0usize;
        let mut cnt_y = 0usize;
        for (jdx, target) in points.iter().enumerate() {
            if jdx == idx {
                continue;
            }
            if target.0 == point.0 {
                cnt_x += 1;
            }
            if target.1 == point.1 {
                cnt_y += 1;
            }
        }

        ans += (cnt_x) * (cnt_y)
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
