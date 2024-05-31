// 카잉 달력

fn get_gcd(a: &usize, b: &usize) -> usize {
    if b > a {
        get_gcd(b, a)
    } else if a % b == 0 {
        *b
    } else {
        get_gcd(&(a % b), b)
    }
}

fn get_lcm(a: &usize, b: &usize) -> usize {
    let gcd = get_gcd(a, b);
    b * a / gcd
}

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    for _ in 0..input.next().unwrap() {
        let m = input.next().unwrap();
        let n = input.next().unwrap();
        let x = input.next().unwrap();
        let y = input.next().unwrap();

        let last = get_lcm(&m, &n);

        for idx in (x..=(last + 2 * m)).step_by(m) {
            if idx > last {
                writeln!(output, "-1").unwrap();
                break;
            }
            if (idx % n == 0 && n == y) || idx % n == y {
                writeln!(output, "{}", idx).unwrap();
                break;
            }
        }
    }
    print!("{}", output);
    Ok(())
}
