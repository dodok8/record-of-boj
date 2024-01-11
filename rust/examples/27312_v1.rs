// 운영진에게 설정 짜기는 어려워

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let m = input.next().unwrap();
    let n = input.next().unwrap();
    let q = input.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut vec_q = vec![1; n + 1];
    let mut vec_a = vec![1; n + 1];
    for idx in 1..(n + 1) {
        let a = input.next().unwrap();
        vec_a[idx] = a;
        if a < m {
            vec_q[idx] = vec_q[idx - 1];
        } else {
            vec_q[idx] = idx;
        }
    }
    for idx in 1..(q + 1) {
        if vec_q[idx] == 1 {
            vec_q[idx] = vec_q[idx - 1];
        }
    }

    let mut available_nums: Vec<HashSet<usize>> = vec![];
    for a in vec_a {
        available_nums.push(HashSet::from_iter(1..(a + 1)));
    }

    for idx in 1..(q + 1) {
        println!("? {} {}", if idx > m { m } else { idx }, vec_q[idx]);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
        let num = input.next().unwrap();
        available_nums[idx].remove(&num);
    }

    write!(output, "! ").unwrap();

    for nums in available_nums.iter().skip(1) {
        if let Some(first) = nums.iter().next() {
            write!(output, "{} ", first).unwrap();
        }
    }
    println!("{}", output);
    Ok(())
}
