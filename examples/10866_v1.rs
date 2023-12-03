// 덱

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let num_command = input.next().unwrap().parse::<usize>().unwrap();
    let mut deque: VecDeque<usize> = VecDeque::new();
    for _ in 0..num_command {
        let command = input.next().unwrap();
        match command {
            "push_front" => {
                let arg = input.next().unwrap().parse::<usize>().unwrap();
                deque.push_front(arg);
            }
            "push_back" => {
                let arg = input.next().unwrap().parse::<usize>().unwrap();
                deque.push_back(arg);
            }
            "pop_front" => {
                if let Some(val) = deque.pop_front() {
                    writeln!(output, "{}", val).unwrap();
                } else {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
            "pop_back" => {
                if let Some(val) = deque.pop_back() {
                    writeln!(output, "{}", val).unwrap();
                } else {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
            "size" => writeln!(output, "{}", deque.len()).unwrap(),
            "empty" => {
                if deque.is_empty() {
                    writeln!(output, "{}", 1).unwrap();
                } else {
                    writeln!(output, "{}", 0).unwrap();
                }
            }
            "front" => {
                if let Some(val) = deque.front() {
                    writeln!(output, "{}", val).unwrap();
                } else {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
            "back" => {
                if let Some(val) = deque.back() {
                    writeln!(output, "{}", val).unwrap();
                } else {
                    writeln!(output, "{}", -1).unwrap();
                }
            }
            _ => return Err("Not supported command".into()),
        }
    }
    print!("{}", output);
    Ok(())
}
