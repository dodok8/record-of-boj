// 절대값 힙
use core::cmp::Ordering;
use core::fmt;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::{Display, Write};
use std::io::{stdin, Read};

#[derive(PartialEq, Eq)]
struct Abs<T>(T);

impl Display for Abs<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for Abs<i32> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Abs<i32> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.abs() == other.0.abs() {
            self.0.cmp(&other.0)
        } else {
            self.0.abs().cmp(&other.0.abs())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut abs_heap: BinaryHeap<Reverse<Abs<i32>>> = BinaryHeap::new();
    let test_count = input.next().unwrap();
    for _ in 0..test_count as usize {
        let given_value = input.next().unwrap();
        if given_value == 0 {
            match abs_heap.pop() {
                Some(pop_value) => {
                    writeln!(output, "{}", pop_value.0).unwrap();
                }
                _ => {
                    writeln!(output, "0").unwrap();
                }
            }
        } else {
            abs_heap.push(Reverse(Abs(given_value)));
        }
    }
    print!("{}", output);
    Ok(())
}
