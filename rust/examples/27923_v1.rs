// 햄버거최대 몇개드실수있나요?

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let num_h = input.next().unwrap();
    let num_c = input.next().unwrap();
    let l = input.next().unwrap();
    let mut cokes = vec![0; num_h];
    let mut hamburgers = vec![];

    for _ in 0..num_h {
        hamburgers.push(input.next().unwrap());
    }
    // 구간 누적합을 구하는 imos 법
    for _ in 0..num_c {
        let t = input.next().unwrap() - 1;
        cokes[t] += 1;
        if t + l <= num_h - 1 {
            cokes[t + l] -= 1;
        }
    }

    for idx in 1..num_h {
        cokes[idx] += cokes[idx - 1];
    }
    cokes.sort_unstable();
    hamburgers.sort_unstable();

    let mut sum = 0;
    for (coke, hamburger) in cokes.iter().zip(hamburgers) {
        sum += hamburger / usize::pow(2, (*coke as u32).min(31));
    }
    writeln!(output, "{}", sum).unwrap();
    print!("{}", output);
    Ok(())
}
