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

    let dist = ((x as f64).powi(2) + (y as f64).powi(2)).sqrt();
    let c = c as f64;

    let ans = if dist == 0.0 {
        0
    } else {
        let q = (dist / (2.0 * c)).floor() as i64; // 2c로 나눈 몫
        let r = dist - (2.0 * c * q as f64); // 나머지

        if r == 0.0 {
            2 * q
        } else if (r - c).abs() < 1e-10 {
            // 이 부분은 처음엔 == 0 였는데 실수 연산 특성 감안해서 수정했습니다.
            2 * q + 1
        } else {
            2 * q + 2
        }
    };

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
