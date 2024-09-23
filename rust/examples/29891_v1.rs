// 체크 포인트 달리기

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let k = input.next().unwrap() as usize;

    let mut neg_pq: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut pos_pq: BinaryHeap<i64> = BinaryHeap::new();

    for _ in 0..n {
        let num = input.next().unwrap();
        if num < 0 {
            neg_pq.push(Reverse(num));
        } else {
            pos_pq.push(num);
        }
    }
    let mut ans = 0;
    while !pos_pq.is_empty() {
        ans += pos_pq.peek().unwrap() * 2;
        let mut checked = k;

        while !pos_pq.is_empty() && checked > 0 {
            pos_pq.pop();
            checked -= 1;
        }
    }

    while !neg_pq.is_empty() {
        ans += neg_pq.peek().unwrap().0.abs() * 2;
        let mut checked = k;

        while !neg_pq.is_empty() && checked > 0 {
            neg_pq.pop();
            checked -= 1;
        }
    }
    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}
