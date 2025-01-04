// 너 그리고 나 (NAVILLERA)

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let x = input.next().unwrap();
    let y = input.next().unwrap();
    let c = input.next().unwrap();

    let dist = ((x.pow(2) + y.pow(2)) as f64).sqrt();
    let mut ans = 0;

    // (0,0)에서 c 만큼의 원을 그리고 다시 c 만큼의 원을 그리고 이렇게 해서 목표 지점에 도달하는데 얼마나 걸리는지 묻는 문제.
    if x == 0 && y == 0 {
        // 아무 것도 할 필요 없음
    } else {
        let left = dist % (c as f64 * 2.0);

        if left == 0.0 || left % (c as f64) == 0.0 {
            ans = (dist / c as f64) as i64;
        } else {
            ans = (dist / (2 * c) as f64) as i64 * 2 + 2;
        }
    }
    writeln!(output, "{}", ans)?;

    print!("{}", output);
    Ok(())
}
