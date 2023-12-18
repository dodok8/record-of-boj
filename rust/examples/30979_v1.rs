// 유치원생 파댕이 돌보기

use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let time = input.next().unwrap();
    let num_candy = input.next().unwrap();
    let mut sum = 0_usize;
    for _ in 0..num_candy {
        sum += input.next().unwrap();
    }
    if sum >= time {
        writeln!(output, "Padaeng_i Happy").unwrap();
    } else {
        writeln!(output, "Padaeng_i Cry").unwrap();
    }
    print!("{}", output);
}
