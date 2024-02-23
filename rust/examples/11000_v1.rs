// 강의실 배정

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut meetings = Vec::new();

    for _ in 0..n {
        meetings.push((input.next().unwrap(), input.next().unwrap()));
    }
    meetings.sort_unstable();
    let mut previous_ends = BinaryHeap::new();
    previous_ends.push(Reverse(meetings[0].1));
    for meeting in meetings.iter().skip(1) {
        let (start, end) = meeting;
        writeln!(output, "{:?}", previous_ends).unwrap();
        if start < &previous_ends.peek().unwrap().0 {
            previous_ends.push(Reverse(*end));
        } else {
            previous_ends.pop();
            previous_ends.push(Reverse(*end));
        }
    }

    writeln!(output, "{}", previous_ends.len()).unwrap();
    print!("{}", output);
    Ok(())
}
